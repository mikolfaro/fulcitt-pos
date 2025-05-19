use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use escpos::{
    driver::FileDriver,
    printer::Printer,
    utils::{DebugMode, Protocol},
};
use log::{error, info, warn};
use printing::{print_tickets, PrintingLayout};
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{App, AppHandle, Manager, State};
use tauri_plugin_store::StoreExt;

use errors::*;
use models::*;

mod errors;
mod models;
mod printing;

type Db = SqlitePool;
#[derive(Clone)]
struct AppState {
    db: Db,
}

type PrinterState = Arc<Mutex<Option<Printer<FileDriver>>>>;

#[tauri::command]
async fn list_products(app_state: State<'_, AppState>) -> CommandResult<Vec<Product>> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT *
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
        INSERT INTO products(name, price, category, is_deleted)
        VALUES ($1, $2, $3, 0)
        ON CONFLICT(name) DO UPDATE SET price = $2, category = $3, is_deleted = 0
    "#,
    )
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
    items: Vec<CartItem>,
) -> CommandResult<i64> {
    if items.is_empty() {
        return Err(CommandError::InvalidInput(
            "Cannot add a sale with no items.".to_string(),
        ));
    }

    let sale_time = format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    let total_amount: f64 = items
        .iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();

    let mut tx = app_state.db.begin().await?;
    let sale_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO sales (sale_time, total_amount) VALUES (?, ?) RETURNING id;
        "#,
    )
    .bind(sale_time)
    .bind(total_amount)
    .fetch_one(&mut *tx)
    .await?;

    for item in &items {
        let quantity = item.quantity;
        let price_at_sale = item.price;

        if quantity <= 0 {
            return Err(CommandError::InvalidInput(format!(
                "Invalid quantity {} for product {}",
                quantity, item.product_id
            )));
        }
        if price_at_sale < 0.0 {
            return Err(CommandError::InvalidInput(format!(
                "Invalid price {} for product {}",
                price_at_sale, item.product_id
            )));
        }

        sqlx::query(
            r#"
            INSERT INTO sale_items (sale_id, product_id, product_name, quantity, price_at_sale)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(sale_id)
        .bind(item.product_id)
        .bind(&item.name)
        .bind(quantity)
        .bind(price_at_sale)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    info!("Created new sale {}", sale_id);

    let mut mutex_guard = printer_state.lock()?;
    let printer = mutex_guard
        .as_mut()
        .ok_or_else(|| CommandError::PrinterNotConfigured)?;
    printer.debug_mode(Some(DebugMode::Dec)).init()?;

    let store = app.get_store("store.json").unwrap();
    let some_store = store.get("ticket-layout");
    let layout = if let Some(store) = some_store {
        serde_json::from_value::<PrintingLayout>(store)?
    } else {
        PrintingLayout::default()
    };

    print_tickets(printer, &layout, sale_id, &items)?;

    Ok(sale_id)
}

#[tauri::command]
async fn get_sales_recap(app_state: State<'_, AppState>) -> CommandResult<Vec<SaleItem>> {
    let item_sales = sqlx::query_as::<_, SaleItem>(
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
    // let sales = sqlx::query_as::<_, Sale>(r#"
    //     SELECT *
    //     FROM sales
    //     LIMIT 10
    // "#)
    //     .fetch_all(&app_state.db)
    //     .await?;

    Ok(vec![])
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
async fn print_last_sale(
    app_state: State<'_, AppState>,
    printer_state: State<'_, PrinterState>,
) -> CommandResult<()> {
    let last_sale = sqlx::query!(
        r#"
        SELECT id
        FROM sales
        ORDER BY id DESC
        LIMIT 1 "#
    )
    .fetch_optional(&app_state.db)
    .await?
    .ok_or_else(|| CommandError::InvalidInput("No sales recorded yet".to_string()))?;

    let last_sale_id = last_sale.id;
    info!("Reprinting tickets of sale {}", last_sale_id);

    let items = sqlx::query_as!(
        CartItem,
        r#"
        SELECT product_id,
            product_name AS name,
            quantity,
            price_at_sale AS price
        FROM sale_items
        WHERE sale_id = ?
    "#,
        last_sale_id
    )
    .fetch_all(&app_state.db)
    .await?;

    let mut mutex_guard = printer_state.lock()?;
    let printer = mutex_guard
        .as_mut()
        .ok_or_else(|| CommandError::PrinterNotConfigured)?;
    let layout = PrintingLayout::default();
    print_tickets(printer, &layout, last_sale_id, &items)?;

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
    device_path: String,
) -> CommandResult<()> {
    info!("Saving printer device {:?}", device_path);

    let store = app
        .get_store("store.json")
        .ok_or(CommandError::StoreSettings)?;

    store.set("printer-device", device_path.clone());

    let path = Path::new(&device_path);
    let driver = FileDriver::open(path)?;
    let new_printer = Printer::new(driver, Protocol::default(), None);
    let mut mutex_guard = printer_state.lock()?;
    *mutex_guard = Some(new_printer);

    Ok(())
}

#[tauri::command]
async fn test_print_raw_file(device_path: String, text_to_print: String) -> CommandResult<()> {
    info!(
        "Attempting to print {:?} on {:?}",
        text_to_print, device_path
    );

    let path = Path::new(&device_path);
    let exists_result = path.try_exists();
    match exists_result {
        Err(e) => {
            error!("Error while checking device path {:?} {:?}", device_path, e);
            return Err(e.into());
        }
        Ok(false) => {
            warn!("Print device path {:?} does not exist", device_path);
            return Err(CommandError::InvalidPrinterDevice);
        }
        Ok(true) => (),
    };

    let driver = FileDriver::open(path)?;
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
        .unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    db
}

fn setup_printer(app: &App) -> Option<Printer<FileDriver>> {
    app.get_store("store.json")
        .and_then(|store| store.get("printer-device"))
        .and_then(|device_path| {
            let path = device_path.to_string();
            let path = Path::new(&path);
            FileDriver::open(path).ok()
        })
        .and_then(|driver| Some(Printer::new(driver, Protocol::default(), None)))
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
            print_last_sale,
            get_print_layout,
            save_print_layout,
            save_printer_device,
            test_print_raw_file,
        ])
        .setup(|app| {
            app.store("store.json")?;

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
