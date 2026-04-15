use std::path::Path;

use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{AppHandle, Builder, Manager, Runtime, State};
use tauri_plugin_opener::open_path;
use tauri_plugin_store::StoreExt;

use errors::*;
use exports::*;
use models::*;
use printer::*;
use storage::*;
use ticket_printing::*;
use uuid::Uuid;

mod errors;
mod exports;
mod models;
pub mod printer;
mod storage;
mod ticket_printing;

use log::{debug, info, warn}; // Use log crate when building application

type Db = SqlitePool;
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

#[derive(Debug, Deserialize, Serialize)]
enum PrinterType {
    None,
    File,
    Network,
    Usb,
}

#[derive(Debug, Deserialize, Serialize)]
struct PrinterId {
    product_id: u16,
    vendor_id: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct PrinterAddress {
    ip: String,
    port: u16,
}

#[derive(Debug, Serialize)]
struct Transaction {
    sale_uuid: Option<Uuid>,
    refund_uuid: Option<Uuid>,
}

#[tauri::command]
async fn list_products(app_state: State<'_, AppState>) -> CommandResult<Vec<Product>> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT id as "id: _", name, category, price, is_deleted, has_deposit
        FROM products
        WHERE is_deleted = 0
    "#
    )
    .fetch_all(&app_state.db)
    .await?;

    Ok(products)
}

#[tauri::command]
async fn create_product(
    product: UnsavedProduct,
    app_state: State<'_, AppState>,
) -> CommandResult<()> {
    sqlx::query(
        r#"
        INSERT INTO products(id, name, price, category, is_deleted, has_deposit)
        VALUES ($1, $2, $3, $4, 0, $5)
        ON CONFLICT(name) DO UPDATE SET price = $3, category = $4, is_deleted = 0, has_deposit = $5
    "#,
    )
    .bind(Uuid::new_v4())
    .bind(&product.name)
    .bind(product.price)
    .bind(&product.category)
    .bind(product.has_deposit)
    .execute(&app_state.db)
    .await?;

    info!("Product {} created", product.name);

    Ok(())
}

#[tauri::command]
async fn update_product(product: Product, app_state: State<'_, AppState>) -> CommandResult<()> {
    sqlx::query(
        r#"
        UPDATE products
        SET name = ?, price = ?, category = ?, has_deposit = ?
        WHERE id = ?
    "#,
    )
    .bind(product.name)
    .bind(product.price)
    .bind(product.category)
    .bind(product.has_deposit)
    .bind(product.id)
    .execute(&app_state.db)
    .await?;

    Ok(())
}

#[tauri::command]
async fn delete_product(product: Product, app_state: State<'_, AppState>) -> CommandResult<()> {
    sqlx::query(
        r#"
        UPDATE products
        SET is_deleted = 1
        WHERE id = ?
    "#,
    )
    .bind(product.id)
    .execute(&app_state.db)
    .await?;

    Ok(())
}

#[tauri::command]
async fn process_transaction<R: Runtime>(
    app: tauri::AppHandle<R>,
    app_state: State<'_, AppState>,
    printer_state: State<'_, NewPrinterState>,
    sale_items: Vec<CartItem>,
    refund_items: Vec<CartItem>,
    payment_method: PaymentMethod,
) -> CommandResult<Transaction> {
    if sale_items.is_empty() && refund_items.is_empty() {
        return Err(CommandError::InvalidInput(
            "pos-messages-cannot-process-transaction-with-no-items".to_string(),
        ));
    }

    // Create sale and refund in a single transaction
    let mut tx = app_state.db.begin().await?;

    let sale_result = if sale_items.is_empty() {
        None
    } else {
        Some(create_sale_with_items(&mut tx, payment_method, &sale_items).await?)
    };

    let refund_result = if refund_items.is_empty() {
        None
    } else {
        Some(create_refund_with_items(&mut tx, &refund_items).await?)
    };

    tx.commit().await?;

    let sale = process_sale_post_commit(app, app_state.clone(), printer_state, sale_result).await?;

    Ok(Transaction {
        sale_uuid: sale.map(|s| s.id),
        refund_uuid: refund_result.map(|r| r.0.id),
    })
}

async fn process_sale_post_commit<R: Runtime>(
    app: tauri::AppHandle<R>,
    app_state: State<'_, AppState>,
    printer_state: State<'_, NewPrinterState>,
    sale_result: Option<(Sale, Vec<SaleItem>)>,
) -> CommandResult<Option<Sale>> {
    let Some((sale, sale_items)) = sale_result else {
        return Ok(None);
    };

    let mut sale_items_for_printing: Vec<(SaleItem, Product)> = vec![];
    for item in sale_items {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id as "id: _", name, category, price, is_deleted, has_deposit
            FROM products
            WHERE id = ?
            "#,
            item.product_id
        )
        .fetch_one(&app_state.db)
        .await?;

        sale_items_for_printing.push((item, product))
    }

    let mut mutex_guard = printer_state.lock().await;
    let managed = mutex_guard
        .as_mut()
        .ok_or(CommandError::PrinterNotConfigured)?;

    managed.refresh_connection()?;

    let store = app.get_store("store.json").unwrap();
    let some_store = store.get("ticket-layout");
    let layout = if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store)?
    } else {
        PrintingLayout::default()
    };

    print_tickets(&mut managed.wrapper, &layout, &sale, &sale_items_for_printing)?;

    Ok(Some(sale))
}

#[derive(Serialize)]
struct Recap {
    sales: Vec<AggregatedSaleItem>,
    refunds: Vec<AggregatedSaleItem>,
}

#[tauri::command]
async fn get_transactions_recap_by_date_range(
    start_date: String,
    end_date: String,
    app_state: State<'_, AppState>,
) -> CommandResult<Recap> {
    let sales = sqlx::query_as!(
        AggregatedSaleItem,
        r#"
            SELECT product_id as "product_id: _",
                product_name,
                SUM(quantity) AS total_quantity_sold,
                SUM(quantity * price_at_sale) AS total_value_sold
            FROM sale_items JOIN sales ON sale_items.sale_id = sales.id
            WHERE date(sales.sale_time) >= date(?) AND date(sales.sale_time) <= date(?)
            GROUP BY product_id;
        "#,
        start_date,
        end_date,
    )
    .fetch_all(&app_state.db)
    .await?;

    let refunds = sqlx::query_as!(
        AggregatedSaleItem,
        r#"
            SELECT
                product_id as "product_id: _",
                product_name,
                SUM(quantity) AS total_quantity_sold,
                SUM(quantity * price_at_refund) AS total_value_sold
            FROM refund_items JOIN refunds ON refund_items.refund_id = refunds.id
            WHERE date(refunds.refund_time) >= date(?) AND date(refunds.refund_time) <= date(?)
            GROUP BY product_id;
        "#,
        start_date,
        end_date
    )
    .fetch_all(&app_state.db)
    .await?;

    Ok(Recap { sales, refunds })
}

#[tauri::command]
async fn get_today_sales(app_state: State<'_, AppState>) -> CommandResult<Vec<Sale>> {
    let now = Utc::now().naive_local();
    let start_of_day = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .ok_or(CommandError::SaleNotFound)?;

    let sales = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: _", number as "number: _", sale_time, total_amount, payment_method as "payment_method: _"
        FROM sales
        WHERE sale_time >= ?
    "#,
        start_of_day
    )
    .fetch_all(&app_state.db)
    .await?;

    Ok(sales)
}

#[tauri::command]
async fn get_sales_by_date_range(
    start_date: String,
    end_date: String,
    app_state: State<'_, AppState>,
) -> CommandResult<Vec<Sale>> {
    info!("Getting sales from {} to {}", start_date, end_date);

    let sales = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: _", number as "number: _", sale_time, total_amount, payment_method as "payment_method: _"
        FROM sales
        WHERE date(sale_time) >= date(?) AND date(sale_time) <= date(?)
        ORDER BY sale_time DESC
    "#,
        start_date,
        end_date
    )
    .fetch_all(&app_state.db)
    .await?;

    Ok(sales)
}

#[tauri::command]
async fn get_all_sales(app_state: State<'_, AppState>) -> CommandResult<Vec<Sale>> {
    info!("Getting all sales");

    let sales = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: _", number as "number: _", sale_time, total_amount, payment_method as "payment_method: _"
        FROM sales
        ORDER BY sale_time DESC
    "#
    )
    .fetch_all(&app_state.db)
    .await?;

    Ok(sales)
}

#[tauri::command]
async fn clear_sales_data(app_state: State<'_, AppState>) -> CommandResult<()> {
    info!("Clearing sales data");

    let mut tx = app_state.db.begin().await?;

    sqlx::query!("DELETE FROM sale_items")
        .execute(&mut *tx)
        .await?;
    sqlx::query!("DELETE FROM sales").execute(&mut *tx).await?;
    sqlx::query!("DELETE FROM refund_items")
        .execute(&mut *tx)
        .await?;
    sqlx::query!("DELETE FROM refunds")
        .execute(&mut *tx)
        .await?;
    sqlx::query!("DELETE FROM products WHERE is_deleted = 1")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

#[tauri::command]
async fn export_sales<R: Runtime>(
    app: AppHandle<R>,
    app_state: State<'_, AppState>,
) -> CommandResult<()> {
    info!("Exporting to XLSX");

    let file_path = app.path().home_dir().unwrap();
    let file_path = file_path.join("export.xlsx");

    export_sales_report(app_state.db.clone(), &file_path).await?;

    open_path(file_path, None::<&str>).unwrap();

    Ok(())
}

#[tauri::command]
async fn print_last_sale<R: Runtime>(
    app: AppHandle<R>,
    app_state: State<'_, AppState>,
    printer_state: State<'_, NewPrinterState>,
) -> CommandResult<()> {
    let last_sale = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: _", number as "number: _", sale_time, total_amount, payment_method as "payment_method: _"
        FROM sales
        ORDER BY sale_time DESC
        LIMIT 1"#
    )
        .fetch_optional(&app_state.db)
        .await?
        .ok_or_else(|| CommandError::InvalidInput("No sales recorded yet".to_string()))?;

    info!("Reprinting tickets of sale {}", last_sale.id);

    let items: Vec<(SaleItem, Product)> =
        get_sale_items_with_products(app_state.db.clone(), last_sale.id).await?;

    let mut mutex_guard = printer_state.lock().await;
    let managed = mutex_guard
        .as_mut()
        .ok_or(CommandError::PrinterNotConfigured)?;

    managed.refresh_connection()?;

    let store = app.get_store("store.json").unwrap();
    let some_store = store.get("ticket-layout");
    let layout = if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store)?
    } else {
        PrintingLayout::default()
    };

    print_tickets(&mut managed.wrapper, &layout, &last_sale, &items)?;

    Ok(())
}

#[tauri::command]
async fn print_sale<R: Runtime>(
    app: AppHandle<R>,
    app_state: State<'_, AppState>,
    printer_state: State<'_, NewPrinterState>,
    sale_id: i64,
) -> CommandResult<()> {
    info!("Reprinting tickets of sale {}", sale_id);

    let sale = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: _", number, sale_time, total_amount, payment_method as "payment_method: _"
        FROM sales
        WHERE id = ?
        "#,
        sale_id
    )
    .fetch_optional(&app_state.db)
    .await?
    .ok_or(CommandError::SaleNotFound)?;

    let items: Vec<(SaleItem, Product)> =
        get_sale_items_with_products(app_state.db.clone(), sale.id).await?;

    let mut mutex_guard = printer_state.lock().await;
    let managed = mutex_guard
        .as_mut()
        .ok_or(CommandError::PrinterNotConfigured)?;

    managed.refresh_connection()?;

    let store = app.get_store("store.json").unwrap();
    let some_store = store.get("ticket-layout");
    let layout = if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store)?
    } else {
        PrintingLayout::default()
    };

    print_tickets(&mut managed.wrapper, &layout, &sale, &items)?;

    Ok(())
}

#[tauri::command]
async fn get_print_layout<R: Runtime>(app: AppHandle<R>) -> CommandResult<PrintingLayout> {
    let store = app
        .get_store("store.json")
        .ok_or(CommandError::LoadSettings)?;

    let some_store = store.get("ticket-layout");

    if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store).map_err(Into::<CommandError>::into)
    } else {
        Ok(PrintingLayout::default())
    }
}

#[tauri::command]
async fn save_print_layout<R: Runtime>(
    app: AppHandle<R>,
    layout: PrintingLayout,
) -> CommandResult<()> {
    info!("Saving updated layout {:?}", layout);

    let store = app
        .get_store("store.json")
        .ok_or(CommandError::StoreSettings)?;

    let layout_value = serde_json::to_value(layout).or(Err(CommandError::StoreSettings))?;
    store.set("ticket-layout", layout_value);

    Ok(())
}

#[tauri::command]
async fn save_printer_selection<R: Runtime>(
    app: AppHandle<R>,
    printer_state: State<'_, NewPrinterState>,
    printer_type: PrinterType,
    printer_id: Option<PrinterId>,
    printer_address: Option<PrinterAddress>,
) -> CommandResult<()> {
    info!(
        "Saving printer {:?}, ID {:?}, IP {:?}",
        printer_type, printer_id, printer_address
    );

    let new_managed = match (&printer_type, &printer_id, &printer_address) {
        (PrinterType::None, _, _) => Err(CommandError::PrinterNotConfigured),
        (PrinterType::File, _, _) => {
            // Create printer output file in app data directory
            let mut print_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app_data_dir");
            print_path.push("printer_output.txt");
            info!("Writing printer output to: {:?}", print_path);
            create_file_printer(&print_path).map(ManagedPrinter::file)
        }
        (PrinterType::Network, _, None) => Err(CommandError::PrinterNotConfigured),
        (PrinterType::Network, _, Some(printer_address)) => {
            ManagedPrinter::network(printer_address.ip.to_string(), printer_address.port)
        }
        (PrinterType::Usb, None, _) => Err(CommandError::PrinterNotConfigured),
        (PrinterType::Usb, Some(printer_id), _) => {
            create_usb_printer(printer_id.vendor_id, printer_id.product_id).map(ManagedPrinter::usb)
        }
    }?;

    let mut mutex_guard = printer_state.lock().await;
    *mutex_guard = Some(new_managed);

    let store = app
        .get_store("store.json")
        .ok_or(CommandError::StoreSettings)?;

    let value = serde_json::to_value(printer_id)?;
    store.set("printer-id", value);

    let value = serde_json::to_value(printer_type)?;
    store.set("printer-type", value);

    let value = serde_json::to_value(printer_address)?;
    store.set("printer-address", value);

    Ok(())
}

#[tauri::command]
async fn list_usb_devices() -> CommandResult<Vec<AvailablePrinter>> {
    // TODO use `list_available_printers` instead
    list_available_usb_devices()
}

#[tauri::command]
async fn test_printer<R: Runtime>(
    app: AppHandle<R>,
    printer_type: PrinterType,
    printer_id: Option<PrinterId>,
    printer_address: Option<PrinterAddress>,
    text_to_print: String,
) -> CommandResult<()> {
    info!(
        "Attempting to print {:?} on {:?}",
        text_to_print, printer_type
    );

    let mut printer = match (printer_type, printer_id, printer_address) {
        (PrinterType::None, _, _) => Err(CommandError::PrinterNotConfigured),
        (PrinterType::File, _, _) => {
            // Create printer output file in app data directory
            let mut print_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app_data_dir");
            print_path.push("printer_output.txt");
            info!("Writing printer output to: {:?}", print_path);
            create_file_printer(&print_path)
        }
        (PrinterType::Network, _, None) => Err(CommandError::PrinterNotConfigured),
        (PrinterType::Network, _, Some(printer_address)) => {
            info!(
                "Test printer: {}:{}",
                printer_address.ip, printer_address.port
            );
            create_network_printer(printer_address.ip.to_string(), printer_address.port)
        }
        (PrinterType::Usb, None, _) => Err(CommandError::PrinterNotConfigured),
        (PrinterType::Usb, Some(printer_id), _) => {
            create_usb_printer(printer_id.vendor_id, printer_id.product_id)
        }
    }?;

    info!("Printer created successfully, sending test print...");
    printer.init()?;
    printer.writeln(&text_to_print)?;
    printer.feed()?;
    printer.print_cut()?;
    info!("Test print completed successfully");

    Ok(())
}

pub async fn setup_db(path: &Path) -> Db {
    if let Err(err) = std::fs::create_dir_all(path) {
        panic!("Error creating directory {}", err);
    }

    let path = path.join("app.db");

    Sqlite::create_database(
        format!(
            "sqlite:{}",
            path.to_str().expect("Path should be something")
        )
        .as_str(),
    )
    .await
    .expect("Failed to create database");

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().expect("Invalid data_dir path"))
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    db
}

fn setup_printer<R: Runtime>(app: &tauri::App<R>) -> Option<ManagedPrinter> {
    app.get_store("store.json").and_then(|store| {
        debug!("Trying to restore saved configs");

        let printer_type_store = store.get("printer-type");
        let printer_type_store = printer_type_store?;
        let printer_type = serde_json::from_value::<PrinterType>(printer_type_store);
        let printer_type = printer_type.unwrap_or(PrinterType::None);

        let printer_id = store
            .get("printer-id")
            .and_then(|printer_id_store| {
                serde_json::from_value::<Option<PrinterId>>(printer_id_store).ok()
            })
            .flatten();

        let printer_address = store
            .get("printer-address")
            .and_then(|printer_address_store| {
                serde_json::from_value::<Option<PrinterAddress>>(printer_address_store).ok()
            })
            .flatten();

        info!(
            "Already configured printer device path found {:?}, ID {:?}, IP {:?}",
            printer_type, printer_id, printer_address
        );

        match (printer_type, printer_id, printer_address) {
            (PrinterType::None, _, _) => None,
            (PrinterType::File, _, _) => {
                let mut print_path = app
                    .path()
                    .app_data_dir()
                    .expect("Failed to get app_data_dir");
                print_path.push("printer_output.txt");

                create_file_printer(&print_path).map(ManagedPrinter::file).ok()
            }
            (PrinterType::Usb, None, _) => {
                warn!("Found USB printer type in config but no printer ID");
                None
            }
            (PrinterType::Usb, Some(printer_id), _) => {
                create_usb_printer(printer_id.vendor_id, printer_id.product_id)
                    .map(ManagedPrinter::usb)
                    .ok()
            }
            (PrinterType::Network, _, None) => {
                warn!("Found network printer type in config but no printer IP");
                None
            }
            (PrinterType::Network, _, Some(printer_address)) => {
                ManagedPrinter::network(printer_address.ip, printer_address.port).ok()
            }
        }
    })
}

pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            list_products,
            create_product,
            update_product,
            delete_product,
            clear_sales_data,
            process_transaction,
            get_transactions_recap_by_date_range,
            get_today_sales,
            get_sales_by_date_range,
            get_all_sales,
            export_sales,
            print_last_sale,
            print_sale,
            get_print_layout,
            save_print_layout,
            save_printer_selection,
            list_usb_devices,
            test_printer,
        ])
        .setup(|app| {
            app.store("store.json")?;

            let printer = setup_printer(app);
            app.manage(tokio::sync::Mutex::new(printer));

            tauri::async_runtime::block_on(async move {
                debug!("Creating DB");
                let path = app.path().app_data_dir().expect("Failed to get data_dir");
                let db = setup_db(path.as_ref()).await;
                app.manage(AppState { db });
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while creating tauri application")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    create_app(Builder::default()).run(|_, _| {})
}
