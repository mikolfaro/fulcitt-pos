use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct UnsavedProduct {
    pub name: String,
    pub category: String,
    pub price: f64,
    pub has_deposit: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Product {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub is_deleted: bool,
    pub has_deposit: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct CartItem {
    pub product_id: Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: i64,
}

#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, sqlx::Type, rust_xlsxwriter::XlsxSerialize,
)]
#[sqlx(rename_all = "PascalCase")]
pub(crate) enum PaymentMethod {
    Card,
    Cash,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct Sale {
    pub id: Uuid,
    pub number: Option<i64>,
    pub sale_time: NaiveDateTime,
    pub total_amount: f64,
    pub payment_method: Option<PaymentMethod>,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct SaleItem {
    pub id: Uuid,
    pub sale_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i64,
    pub price_at_sale: f64,
}

#[derive(Debug, FromRow, Serialize)]
pub(crate) struct AggregatedSaleItem {
    pub product_id: Uuid,
    pub product_name: String,
    pub total_quantity_sold: i64,
    pub total_value_sold: f64,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct Refund {
    pub id: Uuid,
    pub number: Option<i64>,
    pub refund_time: NaiveDateTime,
    pub total_amount: f64,
    pub payment_method: Option<PaymentMethod>,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct RefundItem {
    pub id: Uuid,
    pub refund_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: i64,
    pub price_at_refund: f64,
}
