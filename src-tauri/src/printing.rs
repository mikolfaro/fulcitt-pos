use std::fmt::Debug;

use chrono::Local;
use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{CartItem, CommandResult};

#[derive(Serialize, Deserialize, Debug)]
enum FontSize {
    Small,
    Normal,
    Large,
}
#[derive(Serialize, Deserialize, Debug)]
enum Justify {
    Left,
    Center,
    Righth,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SectionLayout {
    enabled: bool,
    font_size: FontSize,
    justify: Justify,
}

#[derive(Deserialize, Debug, Serialize)]
pub(crate) struct HeaderLayout {
    enabled: bool,
    font_size: FontSize,
    justify: Justify,
    content: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub(crate) struct PrintingLayout {
    header: HeaderLayout,
    body: SectionLayout,
    footer: SectionLayout,
}

pub(crate) fn print_tickets<D>(
    printer: &mut Printer<D>,
    layout: &PrintingLayout,
    sale_id: i64,
    items: &[CartItem],
) -> CommandResult<()>
where
    D: Driver,
{
    info!("Printing tickets for sale {}", sale_id);

    for item in items {
        for i in 0..item.quantity {
            info!(
                "Printing ticket for product {:?} ({} of {})",
                item.name,
                i + 1,
                item.quantity
            );

            if layout.header.enabled {
                print_header(printer, &layout.header, sale_id)?;
            }

            if layout.body.enabled {
                print_body(printer, &layout.body, item)?;
            }

            if layout.footer.enabled {
                print_footer(printer, &layout.footer)?;
            }

            printer.print_cut()?;
        }
    }

    info!("Completed print for sale {}", sale_id);

    Ok(())
}

fn print_header<D>(
    printer: &mut Printer<D>,
    _layout: &HeaderLayout,
    sale_id: i64,
) -> CommandResult<()>
where
    D: Driver,
{
    let sale_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    printer
        .justify(JustifyMode::CENTER)?
        .size(2, 3)?
        .writeln("PICKUP TICKET")?
        .feed()?
        .reset_size()?
        .writeln(&format!("#{} - {}", sale_id, sale_time_str))?;

    Ok(())
}

fn print_body<D>(
    printer: &mut Printer<D>,
    _layout: &SectionLayout,
    item: &CartItem,
) -> CommandResult<()>
where
    D: Driver,
{
    printer
        .justify(JustifyMode::CENTER)?
        .writeln(&item.name.to_string())?
        .feed()?;

    Ok(())
}

fn print_footer<D>(_printer: &mut Printer<D>, _layout: &SectionLayout) -> CommandResult<()>
where
    D: Driver,
{
    Ok(())
}

impl Default for PrintingLayout {
    fn default() -> Self {
        Self {
            header: HeaderLayout {
                enabled: false,
                content: "".into(),
                font_size: FontSize::Normal,
                justify: Justify::Center,
            },
            body: SectionLayout {
                enabled: true,
                font_size: FontSize::Normal,
                justify: Justify::Center,
            },
            footer: SectionLayout {
                enabled: false,
                font_size: FontSize::Normal,
                justify: Justify::Center,
            },
        }
    }
}
