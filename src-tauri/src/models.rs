use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct UnsavedProduct {
    pub name: String,
    pub category: String,
    pub price: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Product {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub is_deleted: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct CartItem {
    pub product_id: Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: i64,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub(crate) struct Sale {
    pub id: Uuid,
    pub sale_time: NaiveDateTime,
    pub total_amount: f64,
    pub payment_method: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
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

#[derive(Debug)]
pub(crate) struct CartItemWithProduct {
    pub quantity: i64,
    pub name_at_sale: String,
    pub price_at_sale: f64,

    pub product_id: Uuid,
    pub name: String,
    pub category: String,
    pub is_product_deleted: bool,
    pub price: f64
}

impl From<CartItemWithProduct> for (CartItem, Product) {
    fn from(value: CartItemWithProduct) -> Self {
        let item = CartItem {
            name: value.name_at_sale,
            price: value.price_at_sale,
            product_id: value.product_id,
            quantity: value.quantity,
        };

        let product = Product {
            id: value.product_id,
            name: value.name,
            category: value.category,
            is_deleted: value.is_product_deleted,
            price: value.price
        };

        (item, product)
    }
}
