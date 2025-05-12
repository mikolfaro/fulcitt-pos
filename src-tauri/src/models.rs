use serde::{Deserialize, Serialize};

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
