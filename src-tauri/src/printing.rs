use chrono::Local;
use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};
use log::info;

use crate::CartItem;

pub(crate) fn print_tickets<D>(
    printer: &mut Printer<D>,
    sale_id: i64,
    items: &[CartItem],
) -> Result<(), String>
where
    D: Driver,
{
    info!("Printing tickets for sale {}", sale_id);

    let sale_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    for item in items {
        for i in 0..item.quantity {
            info!("Printing ticket for product{} ({} of {})", item.name, i + 1, item.quantity);

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

    info!("Completed print for sale {}", sale_id);

    Ok(())
}
