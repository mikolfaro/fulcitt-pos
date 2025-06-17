use std::{collections::HashMap, fmt::Debug};

use escpos::{driver::Driver, printer::Printer, utils::JustifyMode};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{CartItem, CommandResult, Product, Sale};

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
    group_tickets_by_category: bool,
    header: HeaderLayout,
    body: SectionLayout,
    footer: SectionLayout,
}

pub(crate) fn print_tickets<D>(
    printer: &mut Printer<D>,
    layout: &PrintingLayout,
    sale: &Sale,
    items: &[(CartItem, Product)],
) -> CommandResult<()>
where
    D: Driver,
{
    info!("Printing tickets for sale {}", sale.id);

    if layout.group_tickets_by_category {
        print_grouped_tickets(printer, layout, sale, items)?;
    } else {
        print_split_tickets(printer, layout, sale, items)?;
    }

    info!("Completed print for sale {}", sale.id);

    Ok(())
}

fn print_split_tickets<D>(
    printer: &mut Printer<D>,
    layout: &PrintingLayout,
    sale: &Sale,
    items: &[(CartItem, Product)],
) -> CommandResult<()>
where
    D: Driver,
{
    for item in items {
        for i in 0..item.0.quantity {
            info!(
                "Printing ticket for product {:?} ({} of {})",
                item.0.name,
                i + 1,
                item.0.quantity
            );

            if layout.header.enabled {
                info!("Printing header");
                print_header(printer, &layout.header, sale)?;
            }

            info!("Printing body");
            print_body(printer, &layout.body, &item.0)?;

            info!("Printing footer");
            print_footer(printer, &layout.footer, sale)?;

            printer.print_cut()?;
        }
    }

    Ok(())
}

fn print_grouped_tickets<D>(
    printer: &mut Printer<D>,
    layout: &PrintingLayout,
    sale: &Sale,
    items: &[(CartItem, Product)],
) -> CommandResult<()>
where
    D: Driver,
{
    let mut groups: HashMap<String, Vec<CartItem>> = HashMap::new();
    for item in items.iter() {
        if let Some(sale_items) = groups.get_mut(&item.1.category) {
            sale_items.push(item.0.clone());
        } else {
            groups.insert(item.1.category.clone(), vec![item.0.clone()]);
        }
    }

    for (category, items) in groups {
        info!("Printing ticket for group {:?}", category);
        print_header(printer, &layout.header, sale)?;

        for item in items {
            for _ in 0..item.quantity {
                print_body(printer, &layout.body, &item)?;
            }
        }

        print_footer(printer, &layout.footer, sale)?;
    }

    Ok(())
}


fn print_header<D>(
    printer: &mut Printer<D>,
    layout: &HeaderLayout,
    _sale: &Sale,
) -> CommandResult<()>
where
    D: Driver,
{
    if !layout.enabled {
        return Ok(())
    }

    let section_layout: SectionLayout = layout.clone().into();

    with_layout(printer, &section_layout, |p| {
        p.writeln(&layout.content)?;

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
    if !layout.enabled {
        return Ok(())
    }

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
    sale: &Sale,
) -> CommandResult<()>
where
    D: Driver,
{
    if !layout.enabled {
        return Ok(())
    }

    let sale_time_str = sale.sale_time.format("%d-%m-%Y %H:%M:%S").to_string();

    with_layout(printer, layout, |p| {
        p.writeln(&format!("#{} - {}", sale.id, sale_time_str))?;

        Ok(())
    })?
    .feed()?;

    Ok(())
}

fn with_layout<'a, D, F>(
    printer: &'a mut Printer<D>,
    layout: &'a SectionLayout,
    func: F,
) -> CommandResult<&'a mut Printer<D>>
where
    D: Driver,
    F: FnOnce(&mut Printer<D>) -> CommandResult<()>,
{
    match layout.font_size {
        FontSize::Small => printer.size(1, 1)?,
        FontSize::Normal => printer.size(2, 2)?,
        FontSize::Large => printer.size(3, 3)?,
    };

    match layout.justify {
        Justify::Left => printer.justify(JustifyMode::LEFT)?,
        Justify::Center => printer.justify(JustifyMode::CENTER)?,
        Justify::Right => printer.justify(JustifyMode::RIGHT)?,
    };

    func(printer)?;

    printer.reset_size()?;

    Ok(printer)
}

impl Default for PrintingLayout {
    fn default() -> Self {
        Self {
            group_tickets_by_category: false,
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

impl From<HeaderLayout> for SectionLayout {
    fn from(value: HeaderLayout) -> Self {
        Self {
            enabled: value.enabled,
            font_size: value.font_size,
            justify: value.justify,
        }
    }
}
