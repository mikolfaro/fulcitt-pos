use std::path::Path;

use chrono::{Datelike, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{AppHandle, Builder, Manager, Runtime, State};
use tauri_plugin_opener::open_path;
use tauri_plugin_store::StoreExt;

use errors::*;
use exports::*;
use models::*;
use printer::*;
use ticket_printing::*;
use uuid::Uuid;

mod errors;
mod exports;
mod models;
pub mod printer;
mod ticket_printing;

#[cfg(not(test))]
use log::{debug, info, warn}; // Use log crate when building application

#[cfg(test)]
use std::{println as debug, println as info, println as warn};

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
        SELECT id as "id: uuid::Uuid", name, category, price, is_deleted, has_deposit
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
) -> CommandResult<Transaction> {
    if sale_items.is_empty() && refund_items.is_empty() {
        return Err(CommandError::InvalidInput(
            "pos-messages-cannot-process-transaction-with-no-items".to_string(),
        ));
    }

    let mut tx = app_state.db.begin().await?;

    let transaction_time = Local::now().naive_local();
    let start_of_day = NaiveDate::from_ymd_opt(
        transaction_time.year(),
        transaction_time.month(),
        transaction_time.day(),
    )
    .ok_or(CommandError::InvalidInput("Invalid date".to_string()))?
    .and_hms_opt(0, 0, 0)
    .ok_or(CommandError::InvalidInput("Invalid time".to_string()))?;

    let mut sale_uuid: Option<Uuid> = None;
    let mut sale_for_printing: Option<Sale> = None;
    let mut sale_items_for_printing: Vec<(CartItem, Product)> = vec![];

    // Process sale items if any
    if !sale_items.is_empty() {
        // Get the next sale number for today
        let next_sale_number: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(MAX(number), 0) + 1
            FROM sales
            WHERE sale_time >= ?
            "#,
        )
        .bind(start_of_day)
        .fetch_one(&mut *tx)
        .await?;

        let total_amount: f64 = sale_items
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum();

        let sale_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO sales (id, number, sale_time, total_amount) VALUES (?, ?, ?, ?) RETURNING id as "id: uuid::Uuid";
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(Some(next_sale_number))
        .bind(transaction_time)
        .bind(total_amount)
        .fetch_one(&mut *tx)
        .await?;

        let sale = Sale {
            id: sale_id,
            number: Some(next_sale_number),
            payment_method: None,
            sale_time: transaction_time,
            total_amount,
        };

        for item in sale_items {
            let product = sqlx::query_as!(
                Product,
                r#"
                SELECT id as "id: uuid::Uuid", name, category, price, is_deleted, has_deposit
                FROM products
                WHERE id = ?
                "#,
                item.product_id
            )
            .fetch_one(&mut *tx)
            .await?;

            let quantity = item.quantity;
            let price_at_sale = item.price;

            if quantity <= 0 {
                return Err(CommandError::InvalidInput(
                    "pos-messages-invalid-quantity-for-product".to_string(),
                ));
            }
            if price_at_sale < 0.0 {
                return Err(CommandError::InvalidInput(
                    "pos-messages-invalid-price-for-product".to_string(),
                ));
            }

            sqlx::query(
                r#"
                INSERT INTO sale_items (id, sale_id, product_id, product_name, quantity, price_at_sale)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(sale_id)
            .bind(item.product_id)
            .bind(&item.name)
            .bind(quantity)
            .bind(price_at_sale)
            .execute(&mut *tx)
            .await?;

            sale_items_for_printing.push((item, product));
        }

        sale_uuid = Some(sale_id);
        sale_for_printing = Some(sale);
        info!("Created new sale {}", sale_id);
    }

    let mut refund_uuid: Option<Uuid> = None;

    // Process refund items if any
    if !refund_items.is_empty() {
        // Get the next refund number for today
        let next_refund_number: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(MAX(number), 0) + 1
            FROM refunds
            WHERE refund_time >= ?
            "#,
        )
        .bind(start_of_day)
        .fetch_one(&mut *tx)
        .await?;

        let total_amount: f64 = refund_items
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum();

        let refund_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO refunds (id, number, refund_time, total_amount) VALUES (?, ?, ?, ?) RETURNING id as "id: uuid::Uuid";
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(Some(next_refund_number))
        .bind(transaction_time)
        .bind(total_amount)
        .fetch_one(&mut *tx)
        .await?;

        for item in refund_items {
            let product_exists: bool =
                sqlx::query_scalar(r#"SELECT EXISTS FROM products WHERE id = ?"#)
                    .bind(item.product_id)
                    .fetch_one(&mut *tx)
                    .await?;

            if !product_exists {
                return Err(CommandError::InvalidInput(
                    "pos-messages-product-does-not-exist".to_string(),
                ));
            }

            let quantity = item.quantity;
            let price_at_refund = item.price;

            if quantity <= 0 {
                return Err(CommandError::InvalidInput(
                    "pos-messages-invalid-quantity-for-product".to_string(),
                ));
            }

            if price_at_refund < 0.0 {
                return Err(CommandError::InvalidInput(
                    "pos-messages-invalid-price-for-product".to_string(),
                ));
            }

            sqlx::query(
                r#"
                INSERT INTO refund_items (id, refund_id, product_id, product_name, quantity, price_at_refund)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(refund_id)
            .bind(item.product_id)
            .bind(&item.name)
            .bind(quantity)
            .bind(price_at_refund)
            .execute(&mut *tx)
            .await?;
        }

        refund_uuid = Some(refund_id);
        info!("Created new refund {}", refund_id);
    }

    tx.commit().await?;

    // Print receipt for sale if exists (for now, only handle sale printing)
    if let (Some(sale), items) = (sale_for_printing, sale_items_for_printing) {
        // Filter out deposit products from printing
        let items_to_print: Vec<(CartItem, Product)> = items
            .into_iter()
            .filter(|(_, product)| !product.has_deposit)
            .collect();

        if !items_to_print.is_empty() {
            let mut mutex_guard = printer_state.lock().await;
            let printer = mutex_guard
                .as_mut()
                .ok_or(CommandError::PrinterNotConfigured)?;

            let store = app.get_store("store.json").unwrap();
            let some_store = store.get("ticket-layout");
            let layout = if let Some(store) = some_store {
                serde_json::from_value::<PrintingLayout>(store)?
            } else {
                PrintingLayout::default()
            };

            print_tickets(printer, &layout, &sale, &items_to_print)?;
        }
    }

    Ok(Transaction {
        sale_uuid,
        refund_uuid,
    })
}

#[tauri::command]
async fn process_sale<R: Runtime>(
    app: AppHandle<R>,
    app_state: State<'_, AppState>,
    printer_state: State<'_, NewPrinterState>,
    items: Vec<CartItem>,
) -> CommandResult<Uuid> {
    // Wrapper for backward compatibility - calls process_transaction with empty refund_items
    let transaction = process_transaction(
        app,
        app_state,
        printer_state,
        items,
        vec![], // empty refund_items
    )
    .await?;

    // Return the sale_uuid, or error if no sale was created
    transaction.sale_uuid.ok_or(CommandError::InvalidInput(
        "No sale was created".to_string(),
    ))
}

#[tauri::command]
async fn get_sales_recap(app_state: State<'_, AppState>) -> CommandResult<Vec<AggregatedSaleItem>> {
    let item_sales = sqlx::query_as::<_, AggregatedSaleItem>(
        r#"
            SELECT id,
                product_id,
                product_name,
                SUM(quantity) AS total_quantity_sold,
                SUM(quantity * price_at_sale) AS total_value_sold
            FROM sale_items
            GROUP BY product_id;
        "#,
    )
    .fetch_all(&app_state.db)
    .await?;

    Ok(item_sales)
}

#[tauri::command]
async fn get_today_sales(app_state: State<'_, AppState>) -> CommandResult<Vec<Sale>> {
    let now = Utc::now().naive_local();
    let start_of_day = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day())
        .ok_or(CommandError::SaleNotFound)?;

    let sales = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: uuid::Uuid", number as "number: i64", sale_time, total_amount, payment_method
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
async fn clear_sales_data(app_state: State<'_, AppState>) -> CommandResult<()> {
    info!("Clearing sales data");

    let mut tx = app_state.db.begin().await?;

    sqlx::query!("DELETE FROM sale_items")
        .execute(&mut *tx)
        .await?;
    sqlx::query!("DELETE FROM sales").execute(&mut *tx).await?;
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
        SELECT id as "id: uuid::Uuid", number as "number: i64", sale_time, total_amount, payment_method
        FROM sales
        ORDER BY sale_time DESC
        LIMIT 1"#
    )
        .fetch_optional(&app_state.db)
        .await?
        .ok_or_else(|| CommandError::InvalidInput("No sales recorded yet".to_string()))?;

    info!("Reprinting tickets of sale {}", last_sale.id);

    let items: Vec<(CartItem, Product)> = sqlx::query_as!(
        CartItemWithProduct,
        r#"
        SELECT
                sale_items.product_name AS 'name_at_sale',
                sale_items.price_at_sale,
                sale_items.quantity,
                products.id AS 'product_id: uuid::Uuid',
                products.category,
                products.name,
                products.price,
                products.is_deleted AS 'is_product_deleted',
                products.has_deposit
        FROM sale_items
            JOIN products ON sale_items.product_id = products.id
        WHERE sale_id = ?
    "#,
        last_sale.id
    )
    .fetch_all(&app_state.db)
    .await?
    .into_iter()
    .map(|i| i.into())
    .filter(|(_, product): &(CartItem, Product)| !product.has_deposit)
    .collect();

    let mut mutex_guard = printer_state.lock().await;
    let printer = mutex_guard
        .as_mut()
        .ok_or(CommandError::PrinterNotConfigured)?;

    let store = app.get_store("store.json").unwrap();
    let some_store = store.get("ticket-layout");
    let layout = if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store)?
    } else {
        PrintingLayout::default()
    };

    print_tickets(printer, &layout, &last_sale, &items)?;

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
        SELECT id as "id: uuid::Uuid", number, sale_time, total_amount, payment_method
        FROM sales
        WHERE id = ?
        "#,
        sale_id
    )
    .fetch_optional(&app_state.db)
    .await?
    .ok_or(CommandError::SaleNotFound)?;

    let items: Vec<(CartItem, Product)> = sqlx::query_as!(
        CartItemWithProduct,
        r#"
            SELECT
                sale_items.product_name AS "name_at_sale",
                sale_items.price_at_sale,
                sale_items.quantity,
                products.id AS "product_id: uuid::Uuid",
                products.category,
                products.name,
                products.price,
                products.is_deleted AS 'is_product_deleted',
                products.has_deposit
            FROM sale_items
                JOIN products ON sale_items.product_id = products.id
            WHERE sale_id = ?
    "#,
        sale_id
    )
    .fetch_all(&app_state.db)
    .await?
    .into_iter()
    .map(|i| i.into())
    .filter(|(_, product): &(CartItem, Product)| !product.has_deposit)
    .collect();

    let mut mutex_guard = printer_state.lock().await;
    let printer = mutex_guard
        .as_mut()
        .ok_or(CommandError::PrinterNotConfigured)?;

    let store = app.get_store("store.json").unwrap();
    let some_store = store.get("ticket-layout");
    let layout = if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store)?
    } else {
        PrintingLayout::default()
    };

    print_tickets(printer, &layout, &sale, &items)?;

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

    let new_printer = match (&printer_type, &printer_id, &printer_address) {
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
            create_network_printer(printer_address.ip.to_string(), printer_address.port)
        }
        (PrinterType::Usb, None, _) => Err(CommandError::PrinterNotConfigured),
        (PrinterType::Usb, Some(printer_id), _) => {
            create_usb_printer(printer_id.vendor_id, printer_id.product_id)
        }
    }?;

    let mut mutex_guard = printer_state.lock().await;
    *mutex_guard = Some(new_printer);

    let store = app
        .get_store("store.json")
        .ok_or(CommandError::StoreSettings)?;

    let value = serde_json::to_value(printer_id)?;
    store.set("printer-id", value);

    let value = serde_json::to_value(printer_type)?;
    store.set("printer-type", value);

    let value = serde_json::to_value(printer_address)?;
    store.set("printer-ip", value);

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

fn setup_printer<R: Runtime>(app: &tauri::App<R>) -> Option<PrinterWrapper> {
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

                create_file_printer(&print_path).ok()
            }
            (PrinterType::Usb, None, _) => {
                warn!("Found USB printer type in config but no printer ID");
                None
            }
            (PrinterType::Usb, Some(printer_id), _) => {
                create_usb_printer(printer_id.vendor_id, printer_id.product_id).ok()
            }
            (PrinterType::Network, _, None) => {
                warn!("Found network printer type in config but no printer IP");
                None
            }
            (PrinterType::Network, _, Some(printer_address)) => {
                create_network_printer(printer_address.ip, printer_address.port).ok()
            }
        }
    })
}

pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            list_products,
            create_product,
            update_product,
            delete_product,
            clear_sales_data,
            process_sale,
            process_transaction,
            get_sales_recap,
            get_today_sales,
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
                println!("Creating DB");
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
