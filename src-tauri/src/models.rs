use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct UnsavedProduct {
    name: String,
    category: String,
    price: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Product {
    id: i64,
    name: String,
    category: String,
    price: f64,
}

pub(crate) struct CartItem {
    name: String,
    price: String,
    quantity: String
}
