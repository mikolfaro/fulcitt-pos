use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{App, Manager, State};

use models::*;
mod models;

type Db = SqlitePool;
#[derive(Clone)]
struct AppState {
    db: Db,
}

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
async fn add_sale(app_state: State<'_, AppState>, items: Vec<CartItem>) -> Result<i64, String> {
    if items.is_empty() {
        return Err("Cannot add a sale with no items.".to_string());
    }

    let sale_time = format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    let total_amount: f64 = items.iter().map(|item| item.price * item.quantity as f64).sum();

    let mut tx = app_state.db.begin().await.map_err(|e| e.to_string())?;
    let sale_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO sales (sale_time, total_amount) VALUES (?, ?) RETURNING id;
        "#)
        .bind(sale_time)
        .bind(total_amount)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    for item in items {
        let quantity = item.quantity;
        let price_at_sale = item.price;

        if quantity <= 0 {
            return Err(format!("Invalid quantity {} for product {}", quantity, item.product_id));
        }
        if price_at_sale < 0.0 {
            return Err(format!("Invalid price {} for product {}", price_at_sale, item.product_id));
        }

        sqlx::query(
            r#"
            INSERT INTO sale_items (sale_id, product_id, product_name, quantity, price_at_sale)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(sale_id)
            .bind(item.product_id)
            .bind(item.name)
            .bind(quantity)
            .bind(price_at_sale)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    return Ok(sale_id)
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_products,
            create_product,
            update_product,
            delete_product,
            add_sale
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db = setup_db(&app).await;
                app.manage(AppState { db });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
