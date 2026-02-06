use crate::models::{CartItem, PaymentMethod, Product, Refund, RefundItem, Sale, SaleItem};
use crate::{CommandError, CommandResult};

use chrono::{Datelike, Local, NaiveDate};
use log::info;
use sqlx::{Pool, Sqlite, Transaction};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct SaleItemWithProduct {
    pub sale_item_id: Uuid,
    pub quantity: i64,
    pub name_at_sale: String,
    pub price_at_sale: f64,
    pub sale_id: Uuid,

    pub product_id: Uuid,
    pub name: String,
    pub category: String,
    pub is_product_deleted: bool,
    pub price: f64,
    pub has_deposit: bool,
}

impl From<SaleItemWithProduct> for (SaleItem, Product) {
    fn from(value: SaleItemWithProduct) -> Self {
        let item = SaleItem {
            id: value.sale_item_id,
            product_name: value.name_at_sale,
            price_at_sale: value.price_at_sale,
            product_id: value.product_id,
            quantity: value.quantity,
            sale_id: value.sale_id,
        };

        let product = Product {
            id: value.product_id,
            name: value.name,
            category: value.category,
            is_deleted: value.is_product_deleted,
            price: value.price,
            has_deposit: value.has_deposit,
        };

        (item, product)
    }
}

pub async fn create_sale_with_items(
    tx: &mut Transaction<'_, Sqlite>,
    payment_method: PaymentMethod,
    cart_items: &[CartItem],
) -> CommandResult<(Sale, Vec<SaleItem>)> {
    let transaction_time = Local::now().naive_local();
    let start_of_day = NaiveDate::from_ymd_opt(
        transaction_time.year(),
        transaction_time.month(),
        transaction_time.day(),
    )
    .ok_or(CommandError::InvalidInput("Invalid date".to_string()))?
    .and_hms_opt(0, 0, 0)
    .ok_or(CommandError::InvalidInput("Invalid time".to_string()))?;

    // let mut sale_uuid: Option<Uuid> = None;
    // let mut sale_for_printing: Option<Sale> = None;
    // let mut sale_items_for_printing: Vec<(CartItem, Product)> = vec![];

    // Get the next sale number for today
    let next_sale_number: i64 = sqlx::query_scalar(
        r#"
            SELECT COALESCE(MAX(number), 0) + 1
            FROM sales
            WHERE sale_time >= ?
            "#,
    )
    .bind(start_of_day)
    .fetch_one(&mut **tx)
    .await?;

    let total_amount: f64 = cart_items
        .iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();

    let sale_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO sales (id, number, sale_time, total_amount, payment_method) VALUES (?, ?, ?, ?, ?) RETURNING id as "id: uuid::Uuid";
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(Some(next_sale_number))
        .bind(transaction_time)
        .bind(total_amount)
        .bind(Some(payment_method))
        .fetch_one(&mut **tx)
        .await?;

    let sale = Sale {
        id: sale_id,
        number: Some(next_sale_number),
        payment_method: Some(payment_method),
        sale_time: transaction_time,
        total_amount,
    };

    let mut sale_items = vec![];
    for item in cart_items {
        let quantity = item.quantity;
        let price_at_sale = item.price;

        if quantity <= 0 {
            return Err(CommandError::InvalidInput(
                "pos-messages-invalid-quantity-for-product".to_string(),
            ));
        }
        if price_at_sale < 0.0 {
            return Err(CommandError::InvalidInput(
                "pos-messages-invalid-price-for-product".to_string(),
            ));
        }

        let item_id: Uuid = sqlx::query_scalar(
                r#"
                INSERT INTO sale_items (id, sale_id, product_id, product_name, quantity, price_at_sale)
                VALUES (?, ?, ?, ?, ?, ?)
                RETURNING id as "id: uuid::Uuid";
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(sale_id)
            .bind(item.product_id)
            .bind(&item.name)
            .bind(quantity)
            .bind(price_at_sale)
            .fetch_one(&mut **tx)
            .await?;

        sale_items.push(SaleItem {
            id: item_id,
            sale_id,
            price_at_sale,
            product_id: item.product_id,
            product_name: item.name.clone(),
            quantity,
        });
    }

    info!("Created new sale {}", sale_id);

    Ok((sale, sale_items))
}

pub async fn create_refund_with_items(
    tx: &mut Transaction<'_, Sqlite>,
    cart_items: &[CartItem],
) -> CommandResult<(Refund, Vec<RefundItem>)> {
    let transaction_time = Local::now().naive_local();
    let start_of_day = NaiveDate::from_ymd_opt(
        transaction_time.year(),
        transaction_time.month(),
        transaction_time.day(),
    )
    .ok_or(CommandError::InvalidInput("Invalid date".to_string()))?
    .and_hms_opt(0, 0, 0)
    .ok_or(CommandError::InvalidInput("Invalid time".to_string()))?;

    // Get the next refund number for today
    let next_refund_number: i64 = sqlx::query_scalar(
        r#"
            SELECT COALESCE(MAX(number), 0) + 1
            FROM refunds
            WHERE refund_time >= ?
            "#,
    )
    .bind(start_of_day)
    .fetch_one(&mut **tx)
    .await?;

    let total_amount: f64 = cart_items
        .iter()
        .map(|item| item.price * item.quantity as f64)
        .sum();

    let refund_id: Uuid = sqlx::query_scalar(
        r#"
            INSERT INTO refunds (id, number, refund_time, total_amount) VALUES (?, ?, ?, ?) RETURNING id as "id: uuid::Uuid";
            "#,
    )
        .bind(Uuid::new_v4())
        .bind(Some(next_refund_number))
        .bind(transaction_time)
        .bind(total_amount)
        .fetch_one(&mut **tx)
    .await?;

    let refund = Refund {
        id: refund_id,
        number: Some(next_refund_number),
        refund_time: transaction_time,
        payment_method: None,
        total_amount,
    };

    let mut refund_items = vec![];
    for item in cart_items {
        let product_exists: bool =
            sqlx::query_scalar(r#"SELECT EXISTS (SELECT 1 FROM products WHERE id = ?)"#)
                .bind(item.product_id)
                .fetch_one(&mut **tx)
                .await?;

        if !product_exists {
            return Err(CommandError::InvalidInput(
                "pos-messages-product-does-not-exist".to_string(),
            ));
        }

        let quantity = item.quantity;
        let price_at_refund = item.price;

        if quantity <= 0 {
            return Err(CommandError::InvalidInput(
                "pos-messages-invalid-quantity-for-product".to_string(),
            ));
        }

        if price_at_refund < 0.0 {
            return Err(CommandError::InvalidInput(
                "pos-messages-invalid-price-for-product".to_string(),
            ));
        }

        let item_id = sqlx::query_scalar(
                r#"
                INSERT INTO refund_items (id, refund_id, product_id, product_name, quantity, price_at_refund)
                VALUES (?, ?, ?, ?, ?, ?)
                RETURNING id as "id: uuid::Uuid";
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(refund_id)
            .bind(item.product_id)
            .bind(&item.name)
            .bind(quantity)
            .bind(price_at_refund)
            .fetch_one(&mut **tx)
            .await?;

        refund_items.push(RefundItem {
            id: item_id,
            refund_id,
            product_id: item.product_id,
            product_name: item.name.clone(),
            quantity,
            price_at_refund,
        });
    }

    info!("Created new refund {}", refund_id);

    Ok((refund, refund_items))
}

pub async fn get_sale_items_with_products(
    db: Pool<Sqlite>,
    sale_id: Uuid,
) -> CommandResult<Vec<(SaleItem, Product)>> {
    let items: Vec<(SaleItem, Product)> = sqlx::query_as!(
        SaleItemWithProduct,
        r#"
        SELECT
                sale_items.id AS "sale_item_id: _",
                sale_items.product_name AS 'name_at_sale',
                sale_items.price_at_sale,
                sale_items.quantity,
                sale_items.sale_id AS "sale_id: _",

                products.id AS 'product_id: _',
                products.category,
                products.name,
                products.price,
                products.is_deleted AS 'is_product_deleted',
                products.has_deposit
        FROM sale_items
            JOIN products ON sale_items.product_id = products.id
        WHERE sale_id = ?
    "#,
        sale_id
    )
    .fetch_all(&db)
    .await?
    .into_iter()
    .map(|i| i.into())
    .filter(|(_, product): &(SaleItem, Product)| !product.has_deposit)
    .collect();

    Ok(items)
}
