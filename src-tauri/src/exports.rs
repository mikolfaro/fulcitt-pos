use rust_xlsxwriter::{workbook::Workbook, worksheet::Worksheet, Format};
use sqlx::{Pool, Sqlite};

use crate::{CommandResult, Sale, SaleItem};

pub(crate) async fn export_sales_report(
    db: Pool<Sqlite>,
    report_file_path: &str
) -> CommandResult<()> {
    let mut invoices_worksheet = Worksheet::new();
    invoices_worksheet.set_name("reports-export-xslx-invoices-tab-title")?;
    let mut products_worksheet = Worksheet::new();
    products_worksheet.set_name("reports-export-xslx-invoices-details-tab-title")?;

    let sales = sqlx::query_as!(Sale, "SELECT * FROM sales")
        .fetch_all(&db)
    .await?;

    invoices_worksheet.write_row(0, 0, vec!["ID", "Data", "Metodo di pagamento", "Importo"])?;
    products_worksheet.write_row(
        0,
        0,
        vec![
            "ID scontrino",
            "Prodotto",
            "Q.tà",
            "Costo unitario",
            "Totale",
        ],
    )?;

    let currency_format = Format::new().set_num_format("#,##0.00 €");

    for (i, sale) in sales.into_iter().enumerate() {
        let i: u32 = i.try_into().unwrap();

        invoices_worksheet.write(i + 1, 0, sale.id)?;
        invoices_worksheet.write(
            i + 1,
            1,
            sale.sale_time.format("%Y-%M-%d %H:%m:%S").to_string(),
        )?;
        invoices_worksheet.write(i + 1, 3, sale.payment_method)?;
        invoices_worksheet.write_with_format(i + 1, 2, sale.total_amount, &currency_format)?;

        let item_sales = sqlx::query_as!(
            SaleItem,
            r#"
            SELECT *
            FROM sale_items
            WHERE sale_id = ?
            "#,
            sale.id
        )
        .fetch_all(&db)
        .await?;

        let mut j = 1;
        for item in item_sales.into_iter() {
            products_worksheet.write(j, 0, item.sale_id)?;
            products_worksheet.write(j, 1, item.product_name)?;
            products_worksheet.write(j, 2, item.quantity)?;
            products_worksheet.write_with_format(j, 3, item.price_at_sale, &currency_format)?;
            products_worksheet.write_formula_with_format(
                j,
                4,
                format!("=C{}*D{}", j + 1, j + 1).as_str(),
                &currency_format,
            )?;

            j += 1;
        }
    }

    let mut workbook = Workbook::new();
    workbook.push_worksheet(invoices_worksheet);
    workbook.push_worksheet(products_worksheet);
    workbook.save(report_file_path)?;

    Ok(())
}
