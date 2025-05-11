use chrono::Local;
use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};
use log::info;

use crate::{CartItem, CommandResult};

pub(crate) fn print_tickets<D>(
    printer: &mut Printer<D>,
    sale_id: i64,
    items: &[CartItem],
) -> CommandResult<()>
where
    D: Driver,
{
    info!("Printing tickets for sale {}", sale_id);

    let sale_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    for item in items {
        for i in 0..item.quantity {
            info!(
                "Printing ticket for product{} ({} of {})",
                item.name,
                i + 1,
                item.quantity
            );

            printer
                // Header
                .justify(JustifyMode::CENTER)?
                .size(2, 3)?
                .writeln("PICKUP TICKET")?
                .feed()?
                .reset_size()?
                .writeln(&format!("#{} - {}", sale_id, sale_time_str))?
                // Body
                .justify(JustifyMode::CENTER)?
                .writeln(&item.name.to_string())?
                .feed()?
                .cut()?;
        }
    }

    info!("Completed print for sale {}", sale_id);

    Ok(())
}
