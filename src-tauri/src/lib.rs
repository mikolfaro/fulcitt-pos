use std::sync::{Arc, Mutex};

use escpos::{
    driver::ConsoleDriver,
    printer::Printer,
    printer_options::PrinterOptions,
    utils::{DebugMode, Protocol},
};
use log::info;
use printing::print_tickets;
use serde::Serialize;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{App, Manager, State};

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

type PrinterState = Arc<Mutex<Printer<ConsoleDriver>>>;

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
    .bind(&product.price)
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

    let mut printer = printer_state.lock()?;
    printer.debug_mode(Some(DebugMode::Dec)).init()?;

    print_tickets(&mut *printer, sale_id, &items)?;

    Ok(sale_id)
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct ItemSale {
    product_id: i64,
    product_name: String,
    total_quantity_sold: i64,
    total_value_sold: f64,
}

#[tauri::command]
async fn get_sales_recap(app_state: State<'_, AppState>) -> CommandResult<Vec<ItemSale>> {
    let item_sales = sqlx::query_as::<_, ItemSale>(
        r#"
            SELECT product_id,
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
async fn test_print_raw_file(
    printer_state: State<'_, PrinterState>,
    text_to_print: String,
) -> CommandResult<()> {
    info!("Attempting to print '{}'", text_to_print);

    let mut printer = printer_state.lock()?;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
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
