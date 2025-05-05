use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use tauri::{App, Manager, State};

type Db = SqlitePool;
#[derive(Clone)]
struct AppState { db: Db }

#[derive(Deserialize)]
struct UnsavedProduct {
    name: String,
    category: String,
    price: f64
}

#[derive(Serialize)]
struct Product {
    id: i64,
    name: String,
    category: String,
    price: f64
}

#[tauri::command]
async fn list_products(app_state: State<'_, AppState>) -> Result<Vec<Product>, String> {
    let products = sqlx::query_as!(Product, r#"
        SELECT * FROM products
    "#).fetch_all(&app_state.db)
        .await.map_err(|e| e.to_string())?;

    return Ok(products)
}

#[tauri::command]
async fn create_product(product: UnsavedProduct, app_state: State<'_, AppState>) -> Result<(), String> {
    sqlx::query(r#"
        INSERT INTO products(name, price, category)
        VALUES (?, ?, ?)
    "#)
        .bind(product.name)
        .bind(product.price)
        .bind(product.category)
        .execute(&app_state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn setup_db(app: &App) -> Db {
    let mut path = app.path().app_data_dir().expect("Failed to get data_dir");
    if let Err(err) = std::fs::create_dir_all(path.clone()) {
        panic!("Error creating directory {}", err);
    }

    path.push("app.db");

    Sqlite::create_database(format!("sqlite:{}", path.to_str().expect("Path should be something")).as_str())
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
            create_product
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
