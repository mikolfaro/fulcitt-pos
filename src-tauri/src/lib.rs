use std::sync::{Arc, Mutex};

use escpos::{
    driver::ConsoleDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{DebugMode, Protocol},
};
use printing::print_tickets;
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{App, Manager, State};

use models::*;

mod models;
mod printing;

type Db = SqlitePool;
#[derive(Clone)]
struct AppState {
    db: Db,
}

type PrinterState = Arc<Mutex<Printer<ConsoleDriver>>>;

#[tauri::command]
async fn list_products(app_state: State<'_, AppState>) -> Result<Vec<Product>, String> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT * FROM products
    "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(products)
}

#[tauri::command]
async fn create_product(
    product: UnsavedProduct,
    app_state: State<'_, AppState>,
) -> Result<(), String> {
    sqlx::query(
        r#"
        INSERT INTO products(name, price, category)
        VALUES (?, ?, ?)
    "#,
    )
    .bind(product.name)
    .bind(product.price)
    .bind(product.category)
    .execute(&app_state.db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn update_product(product: Product, app_state: State<'_, AppState>) -> Result<(), String> {
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
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn delete_product(product: Product, app_state: State<'_, AppState>) -> Result<(), String> {
    sqlx::query(
        r#"
        DELETE FROM products WHERE id = ?
    "#,
    )
    .bind(product.id)
    .execute(&app_state.db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn process_sale(
    app_state: State<'_, AppState>,
    printer_state: State<'_, PrinterState>,
    items: Vec<CartItem>,
) -> Result<i64, String> {
    if items.is_empty() {
        return Err("Cannot add a sale with no items.".to_string());
    }

    let sale_time = format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    let total_amount: f64 = items
        .iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();

    let mut tx = app_state.db.begin().await.map_err(|e| e.to_string())?;
    let sale_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO sales (sale_time, total_amount) VALUES (?, ?) RETURNING id;
        "#,
    )
    .bind(sale_time)
    .bind(total_amount)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    for item in &items {
        let quantity = item.quantity;
        let price_at_sale = item.price;

        if quantity <= 0 {
            return Err(format!(
                "Invalid quantity {} for product {}",
                quantity, item.product_id
            ));
        }
        if price_at_sale < 0.0 {
            return Err(format!(
                "Invalid price {} for product {}",
                price_at_sale, item.product_id
            ));
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
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    let mut printer = printer_state.lock().unwrap();
    printer.init().map_err(|e| e.to_string())?;

    print_tickets(&mut *printer, sale_id, &items)?;

    Ok(sale_id)
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct ItemSale {
    product_id: i64,
    product_name: String,
    total_quantity_sold: i64,
    total_value_sold: f64
}

#[tauri::command]
async fn get_sales_recap(
    app_state: State<'_, AppState>
) -> Result<Vec<ItemSale>, String> {
    let item_sales = sqlx::query_as::<_, ItemSale>(
        r#"
            SELECT product_id,
                product_name,
                SUM(quantity) AS total_quantity_sold,
                SUM(quantity * price_at_sale) AS total_value_sold
            FROM sale_items
            GROUP BY product_id;
        "#
    )
        .fetch_all(&app_state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(item_sales)
}

#[tauri::command]
async fn test_print_raw_file(
    printer_state: State<'_, PrinterState>,
    text_to_print: String,
) -> Result<(), String> {
    println!("Attempting to print '{}'", text_to_print);

    let mut printer = printer_state.lock().unwrap();

    println!();
    printer
        .debug_mode(Some(DebugMode::Hex))
        .init()
        .map_err(|_| "Failed to initialize printer")?
        .writeln(&text_to_print)
        .map_err(|_| "Failed to write ln")?
        .feed()
        .map_err(|_| "Failed to feed")?
        .print_cut()
        .map_err(|_| "Failed to cut")?;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_products,
            create_product,
            update_product,
            delete_product,
            process_sale,
            get_sales_recap,
            test_print_raw_file,
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db = setup_db(app).await;
                app.manage(AppState { db });

                let driver = ConsoleDriver::open(true);
                let printer =
                    Printer::new(driver, Protocol::default(), Some(PrinterOptions::default()));
                let printer_state = Arc::new(Mutex::new(printer));
                app.manage(printer_state);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
