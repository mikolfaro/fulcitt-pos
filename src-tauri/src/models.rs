use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub(crate) struct UnsavedProduct {
    pub name: String,
    pub category: String,
    pub price: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Product {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub is_deleted: bool,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CartItem {
    pub product_id: i64,
    pub name: String,
    pub price: f64,
    pub quantity: i64,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct Sale {
    pub id: i64,
    pub sale_time: NaiveDateTime,
    pub total_amount: f64,
    pub payment_method: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub(crate) struct SaleItem {
    id: i64,
    product_id: i64,
    product_name: String,
    total_quantity_sold: i64,
    total_value_sold: f64,
}
