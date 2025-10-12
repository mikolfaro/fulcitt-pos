use std::sync::{Arc, Mutex};

use chrono::{Datelike, Local, NaiveDate, Utc};
use escpos::{
    printer::Printer,
    utils::{DebugMode, Protocol},
};

#[cfg(debug_assertions)]
use escpos::driver::ConsoleDriver;
#[cfg(not(debug_assertions))]
use escpos::driver::UsbDriver;

use log::info;
use printing::{print_tickets, PrintingLayout};
use rusb::{Context, DeviceList};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{App, AppHandle, Manager, State};
use tauri_plugin_store::StoreExt;
use unic_langid::langid;

use errors::*;
use exports::*;
use intl::*;
use models::*;
use uuid::Uuid;

mod errors;
mod exports;
mod intl;
mod models;
mod printing;

type Db = SqlitePool;
#[derive(Clone)]
struct AppState {
    db: Db,
}

#[cfg(debug_assertions)]
type PrinterState = Arc<Mutex<Option<Printer<ConsoleDriver>>>>;

#[cfg(not(debug_assertions))]
type PrinterState = Arc<Mutex<Option<Printer<UsbDriver>>>>;

#[derive(Debug, Serialize, Deserialize)]
struct UsbDevice {
    vendor_id: u16,
    product_id: u16,
    vendor_name: String,
    product_name: String,
}

#[tauri::command]
async fn list_products(app_state: State<'_, AppState>) -> CommandResult<Vec<Product>> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT id as "id: uuid::Uuid", name, category, price, is_deleted
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
        INSERT INTO products(id, name, price, category, is_deleted)
        VALUES ($1, $2, $3, $4, 0)
        ON CONFLICT(name) DO UPDATE SET price = $3, category = $4, is_deleted = 0
    "#,
    )
    .bind(Uuid::new_v4())
    .bind(&product.name)
    .bind(product.price)
    .bind(&product.category)
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
        SET name = ?, price = ?, category = ?
        WHERE id = ?
    "#,
    )
    .bind(product.name)
    .bind(product.price)
    .bind(product.category)
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
async fn process_sale(
    app: AppHandle,
    app_state: State<'_, AppState>,
    printer_state: State<'_, PrinterState>,
    intl_state: State<'_, Intl>,
    items: Vec<CartItem>,
) -> CommandResult<Uuid> {
    if items.is_empty() {
        return Err(CommandError::InvalidInput(
            intl_state
                .t("pos-messages-cannot-process-sale-with-no-items")?
                .to_string(),
        ));
    }

    let sale_time = Local::now().naive_local();

    let total_amount: f64 = items
        .iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();

    let mut tx = app_state.db.begin().await?;
    let sale_id: uuid::Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO sales (id, sale_time, total_amount) VALUES (?, ?, ?) RETURNING id as "id: uuid::Uuid";
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(sale_time)
    .bind(total_amount)
    .fetch_one(&mut *tx)
    .await?;

    let sale = Sale {
        id: sale_id,
        payment_method: None,
        sale_time,
        total_amount,
    };

    let mut items_with_products: Vec<(CartItem, Product)> = vec!();
    for item in items {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id as "id: uuid::Uuid", name, category, price, is_deleted
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
                intl_state
                    .t("pos-messages-invalid-quantity-for-product")?
                    .to_string(), // quantity, item.product_id
            ));
        }
        if price_at_sale < 0.0 {
            return Err(CommandError::InvalidInput(
                intl_state
                    .t("pos-messages-invalid-price-for-product")?
                    .to_string(), // price_at_sale, item.product_id
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

        items_with_products.push((item, product));
    }

    tx.commit().await?;

    info!("Created new sale {}", sale_id);

    let mut mutex_guard = printer_state.lock()?;
    let printer = mutex_guard
        .as_mut()
        .ok_or(CommandError::PrinterNotConfigured)?;
    printer.debug_mode(Some(DebugMode::Dec)).init()?;

    let store = app.get_store("store.json").unwrap();
    let some_store = store.get("ticket-layout");
    let layout = if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store)?
    } else {
        PrintingLayout::default()
    };

    print_tickets(printer, &layout, &sale, &items_with_products)?;

    Ok(sale_id)
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
        SELECT id as "id: uuid::Uuid", sale_time, total_amount, payment_method
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
async fn export_sales(app_state: State<'_, AppState>) -> CommandResult<()> {
    info!("Exporting to XLSX");

    export_sales_report(app_state.db.clone(), "/home/mikol/export.xlsx").await
}

#[tauri::command]
async fn print_last_sale(
    app: AppHandle,
    app_state: State<'_, AppState>,
    printer_state: State<'_, PrinterState>,
) -> CommandResult<()> {
    let last_sale = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: uuid::Uuid", sale_time, total_amount, payment_method
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
                products.is_deleted AS 'is_product_deleted'
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
        .collect();

    let mut mutex_guard = printer_state.lock()?;
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
async fn print_sale(
    app: AppHandle,
    app_state: State<'_, AppState>,
    printer_state: State<'_, PrinterState>,
    sale_id: i64,
) -> CommandResult<()> {
    info!("Reprinting tickets of sale {}", sale_id);

    let sale = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: uuid::Uuid", sale_time, total_amount, payment_method
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
                products.is_deleted AS 'is_product_deleted'
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
        .collect();


    let mut mutex_guard = printer_state.lock()?;
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
async fn get_print_layout(app: AppHandle) -> CommandResult<PrintingLayout> {
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
async fn save_print_layout(layout: PrintingLayout, app: AppHandle) -> CommandResult<()> {
    info!("Saving updated layout {:?}", layout);

    let store = app
        .get_store("store.json")
        .ok_or(CommandError::StoreSettings)?;

    let layout_value = serde_json::to_value(layout).or(Err(CommandError::StoreSettings))?;
    store.set("ticket-layout", layout_value);

    Ok(())
}

#[tauri::command]
async fn save_printer_device(
    app: AppHandle,
    printer_state: State<'_, PrinterState>,
    device: UsbDevice,
) -> CommandResult<()> {
    info!("Saving printer device {:?}", device);

    #[cfg(debug_assertions)]
    let driver = ConsoleDriver::open(true);
    #[cfg(not(debug_assertions))]
    let driver = UsbDriver::open(device.vendor_id, device.product_id, None)?;

    let new_printer = Printer::new(driver, Protocol::default(), None);
    let mut mutex_guard = printer_state.lock()?;
    *mutex_guard = Some(new_printer);

    let store = app
        .get_store("store.json")
        .ok_or(CommandError::StoreSettings)?;

    let value = serde_json::to_value(device)?;
    store.set("printer-device", value);

    Ok(())
}

fn get_string_descriptor(device: &rusb::Device<rusb::Context>, index: Option<u8>) -> Option<String> {
    if index.is_none() || index == Some(0) {
        return None;
    }

    let index = index.expect("Index must be known now");

    let handle = device.open().ok()?;
    let timeout = std::time::Duration::from_secs(1);
    let languages = handle.read_languages(timeout).ok()?;

    if let Some(language) = languages.first() {
        handle.read_string_descriptor(*language, index, timeout).ok()
    } else {
        None
    }
}

#[tauri::command]
async fn list_usb_devices(
) -> CommandResult<Vec<UsbDevice>> {
    let context = Context::new()?;
    let devices = DeviceList::new_with_context(context)?;

    let mut usb_devices = Vec::new();

    for device in devices.iter() {
        let device_desc = device.device_descriptor()?;

        usb_devices.push(UsbDevice {
            vendor_id: device_desc.vendor_id(),
            product_id: device_desc.product_id(),
            vendor_name: get_string_descriptor(&device, device_desc.manufacturer_string_index()).unwrap_or_default(),
            product_name: get_string_descriptor(&device, device_desc.product_string_index()).unwrap_or_default(),
        });
    }

    Ok(usb_devices)
}


#[tauri::command]
async fn test_print_raw_file(device: UsbDevice, text_to_print: String) -> CommandResult<()> {
    info!("Attempting to print {:?} on {:?}", text_to_print, device);

    let driver = UsbDriver::open(device.vendor_id, device.product_id, None)?;
    let mut printer = Printer::new(driver, Protocol::default(), None);

    println!();
    printer
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln(&text_to_print)?
        .feed()?
        .print_cut()?;
    println!();

    Ok(())
}

async fn setup_db(app: &App) -> Db {
    let mut path = app.path().app_data_dir().expect("Failed to get data_dir");
    if let Err(err) = std::fs::create_dir_all(path.clone()) {
        panic!("Error creating directory {}", err);
    }

    path.push("app.db");

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
        .connect(path.to_str().unwrap())
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    db
}

#[cfg(debug_assertions)]
fn setup_printer(_app: &App) -> Option<Printer<ConsoleDriver>> {
    let driver = ConsoleDriver::open(true);
    Some(Printer::new(driver, Protocol::default(), None))
}

#[cfg(not(debug_assertions))]
fn setup_printer(app: &App) -> Option<Printer<UsbDriver>> {
    app.get_store("store.json")
        .and_then(|store| store.get("printer-device"))
        .and_then(|device| {
            info!("Already configured printer device path found {}", device);
            let device = serde_json::from_value::<UsbDevice>(device).ok()?;

            UsbDriver::open(device.vendor_id, device.product_id, None).ok()
        })
        .map(|driver| {
            debug!("Existing printer restored");
            Printer::new(driver, Protocol::default(), None)
        })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            get_sales_recap,
            get_today_sales,
            export_sales,
            print_last_sale,
            print_sale,
            get_print_layout,
            save_print_layout,
            save_printer_device,
            list_usb_devices,
            test_print_raw_file,
        ])
        .setup(|app| {
            app.store("store.json")?;

            let langid_it = langid!("it");
            let intl = Intl::try_new(langid_it).expect("Failed to load localization");
            app.manage(intl);

            let printer = setup_printer(app);
            app.manage(Arc::new(Mutex::new(printer)));

            tauri::async_runtime::block_on(async move {
                let db = setup_db(app).await;
                app.manage(AppState { db });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
