use fulcitt_pos_lib::create_app;
use tauri::test::mock_builder;

#[test]
fn create_sale_from_transaction() {
    let app = create_app(mock_builder());
}

#[test]
fn create_refund_from_transaction() {}
