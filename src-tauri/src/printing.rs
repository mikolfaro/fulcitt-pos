use chrono::Local;
use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};

use crate::CartItem;

pub(crate) fn print_tickets<D>(
    sale_id: i64,
    items: &[CartItem],
    mut printer: Printer<D>,
) -> Result<(), String>
where
    D: Driver,
{
    let sale_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    println!("Printing");

    for item in items {
        for _ in 0..item.quantity {
            printer
                // Header
                .justify(JustifyMode::CENTER)
                .and_then(|p| p.size(2, 3))
                .and_then(|p| p.writeln("PICKUP TICKET"))
                .and_then(|p| p.feed())
                .and_then(|p| p.reset_size())
                .and_then(|p| p.writeln(&format!("#{} - {}", sale_id, sale_time_str)))
                // Body
                .and_then(|p| p.justify(JustifyMode::CENTER))
                .and_then(|p| p.writeln(&item.name.to_string()))
                .and_then(|p| p.feed())
                .and_then(|p| p.cut())
                .map_err(|e| e.to_string())?;
        }
    }

    println!("Completed print");

    Ok(())
}
