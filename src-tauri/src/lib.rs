use escpos::{
    driver::ConsoleDriver,
    printer::Printer,
    utils::{DebugMode, Protocol},
};
use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
async fn test_print_raw_file(device_path: String, text_to_print: String) -> Result<(), String> {
    println!(
        "Attempting to print '{}' to device '{}'",
        text_to_print, device_path
    );

    let driver = ConsoleDriver::open(true);
    Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Hex))
        .init()
        .map_err(|_| "Failed to initialize printer")?
        .writeln(&text_to_print)
        .map_err(|_| "Failed to write ln")?
        .feed()
        .map_err(|_| "Failed to feed")?
        .print_cut()
        .map_err(|_| "Failed to cut")?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "create_products_table",
        sql: "
                CREATE TABLE IF NOT EXISTS products (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL UNIQUE,
                    price REAL NOT NULL CHECK(price >= 0),
                    category TEXT NOT NULL
                );
                ",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:app.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![test_print_raw_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
