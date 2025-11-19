use std::path::Path;

use fulcitt_pos_lib::{create_app, printer::PrinterWrapper, setup_db, AppState};
use serde_json::json;
use tauri::test::{get_ipc_response, mock_builder};
use tauri::{test::MockRuntime, App, Manager};

fn create_test_app() -> App<MockRuntime> {
    let app = create_app(mock_builder());

    let app_handle = app.handle();
    tauri::async_runtime::block_on(async move {
        let db = setup_db(Path::new("../data")).await;
        app_handle.manage(AppState { db });
    });

    app.manage(tokio::sync::Mutex::new(None::<PrinterWrapper>));

    app
}

#[test]
fn fail_transaction_creation_with_empty_items() {
    let app = create_test_app();
    let webview = tauri::WebviewWindowBuilder::new(&app, "test", Default::default())
        .build()
        .unwrap();

    let body = json!({ "refundItems": [], "saleItems": [] });

    let result = get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "process_transaction".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    );

    assert!(result.is_err());
}

#[test]
fn create_refund_from_transaction() {}
