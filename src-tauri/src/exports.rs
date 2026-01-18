use std::path::Path;

use log::debug;
use rust_xlsxwriter::{workbook::Workbook, worksheet::Worksheet, Format};
use sqlx::{Pool, Sqlite};

use crate::models::PaymentMethod;
use crate::{CommandResult, Refund, Sale};

#[derive(Debug)]
struct SaleSummary {
    product_name: String,
    price_at_sale: f64,
    quantity: i64,
}

pub(crate) async fn export_sales_report(
    db: Pool<Sqlite>,
    report_file_path: &Path,
) -> CommandResult<()> {
    let currency_format = Format::new().set_num_format("#,##0.00 €");

    let mut products_worksheet = Worksheet::new();
    products_worksheet.set_name("venduto netto")?;

    let sale_summaries = sqlx::query_as!(
        SaleSummary,
        r#"
        SELECT product_name, price_at_sale, SUM(quantity) as quantity FROM (
            SELECT product_id, product_name, price_at_sale, quantity FROM sale_items
            UNION ALL
            SELECT product_id, product_name, price_at_refund as price_at_sale, -quantity as quantity FROM refund_items
        )
        GROUP BY product_id, price_at_sale
        HAVING SUM(quantity) != 0
        "#,
    )
        .fetch_all(&db)
        .await?;

    products_worksheet.write_row(0, 0, vec!["Nome", "Quantità", "Prezzo unitario", "Totale"])?;

    for (i, sale_summary) in sale_summaries.into_iter().enumerate() {
        let i: u32 = i.try_into().unwrap();

        debug!("Product line {:?}", sale_summary);

        products_worksheet.write(i + 1, 0, sale_summary.product_name)?;
        products_worksheet.write_with_format(
            i + 1,
            1,
            sale_summary.price_at_sale,
            &currency_format,
        )?;

        products_worksheet.write(i + 1, 2, sale_summary.quantity)?;
        products_worksheet.write_with_format(
            i + 1,
            3,
            sale_summary.price_at_sale * sale_summary.quantity as f64,
            &currency_format,
        )?;
    }

    // Invoices worksheet
    let mut invoices_worksheet = Worksheet::new();
    invoices_worksheet.set_name("scontrini")?;

    let sales = sqlx::query_as!(
        Sale,
        r#"
        SELECT id as "id: _", number as "number: i64", sale_time, total_amount, payment_method as "payment_method: _"
        FROM sales
        ORDER BY sale_time DESC
        "#,
    )
        .fetch_all(&db)
        .await?;

    invoices_worksheet.write_row(
        0,
        0,
        vec!["Numero", "Data", "Metodo di pagamento", "Importo"],
    )?;

    let currency_format = Format::new().set_num_format("#,##0.00 €");

    for (i, sale) in sales.into_iter().enumerate() {
        let i: u32 = i.try_into().unwrap();

        invoices_worksheet.write(i + 1, 0, sale.number)?;
        invoices_worksheet.write(
            i + 1,
            1,
            sale.sale_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        )?;

        match sale.payment_method {
            Some(PaymentMethod::Card) => invoices_worksheet.write(i + 1, 2, "Carta")?,
            Some(PaymentMethod::Cash) => invoices_worksheet.write(i + 1, 2, "Contanti")?,
            None => &invoices_worksheet,
        };

        invoices_worksheet.write_with_format(i + 1, 3, sale.total_amount, &currency_format)?;
    }

    // Refunds worksheet
    let mut refunds_worksheet = Worksheet::new();
    refunds_worksheet.set_name("rimborsi e storni")?;

    let refunds = sqlx::query_as!(
        Refund,
        r#"
        SELECT id as "id: _", number as "number: _", refund_time, total_amount, payment_method as "payment_method: _"
        FROM refunds
        ORDER BY refund_time DESC
        "#,
    )
    .fetch_all(&db)
    .await?;

    refunds_worksheet.write_row(
        0,
        0,
        vec!["Numero", "Data", "Metodo di pagamento", "Importo"],
    )?;

    for (i, refund) in refunds.into_iter().enumerate() {
        let i: u32 = i.try_into().unwrap();

        refunds_worksheet.write(i + 1, 0, refund.number)?;
        refunds_worksheet.write(
            i + 1,
            1,
            refund.refund_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        )?;

        match refund.payment_method {
            Some(PaymentMethod::Card) => refunds_worksheet.write(i + 1, 2, "Carta")?,
            Some(PaymentMethod::Cash) => refunds_worksheet.write(i + 1, 2, "Contanti")?,
            None => &refunds_worksheet,
        };

        refunds_worksheet.write_with_format(i + 1, 3, refund.total_amount, &currency_format)?;
    }

    let mut workbook = Workbook::new();
    workbook.push_worksheet(products_worksheet);
    workbook.push_worksheet(invoices_worksheet);
    workbook.push_worksheet(refunds_worksheet);
    workbook.save(report_file_path)?;

    Ok(())
}
