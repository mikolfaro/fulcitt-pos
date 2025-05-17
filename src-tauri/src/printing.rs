use std::fmt::Debug;

use chrono::Local;
use escpos::{driver::Driver, printer::Printer};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{CartItem, CommandResult};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum FontSize {
    Small,
    Normal,
    Large,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
enum Justify {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SectionLayout {
    enabled: bool,
    font_size: FontSize,
    justify: Justify,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub(crate) struct HeaderLayout {
    enabled: bool,
    font_size: FontSize,
    justify: Justify,
    content: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
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
                print_footer(printer, &layout.footer, sale_id)?;
            }

            printer.print_cut()?;
        }
    }

    info!("Completed print for sale {}", sale_id);

    Ok(())
}

fn print_header<D>(
    printer: &mut Printer<D>,
    layout: &HeaderLayout,
    _sale_id: i64,
) -> CommandResult<()>
where
    D: Driver,
{
    let section_layout: SectionLayout = layout.clone().into();

    with_layout(printer, &section_layout, |p| {
        p
            .writeln("PICKUP TICKET")?;

        Ok(())
    })?
        .feed()?;

    Ok(())
}

fn print_body<D>(
    printer: &mut Printer<D>,
    layout: &SectionLayout,
    item: &CartItem,
) -> CommandResult<()>
where
    D: Driver,
{
    with_layout(printer, layout, |p| {
        p.writeln(&item.name.to_string())?;

        Ok(())
    })?
        .feed()?;

    Ok(())
}

fn print_footer<D>(
    printer: &mut Printer<D>,
    layout: &SectionLayout,
    sale_id: i64,
) -> CommandResult<()>
where
    D: Driver,
{
    let sale_time_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    with_layout(printer, layout, |p | {
        p.writeln(&format!("#{} - {}", sale_id, sale_time_str))?;

        Ok(())
    })?
        .feed()?;

    Ok(())
}

fn with_layout<'a, D, F>(
    printer: &'a mut Printer<D>,
    _layout: &'a SectionLayout,
    func: F
) -> CommandResult<&'a mut Printer<D>>
where
    D: Driver,
    F: FnOnce(&mut Printer<D>) -> CommandResult<()>
{
    // TODO: set font size
    // TODO: set alignement

    func(printer)?;

    printer
        .reset_size()?;

    Ok(printer)
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

impl Into<SectionLayout> for HeaderLayout {
    fn into(self) -> SectionLayout {
        SectionLayout {
            enabled: self.enabled,
            font_size: self.font_size,
            justify: self.justify
        }
    }
}
