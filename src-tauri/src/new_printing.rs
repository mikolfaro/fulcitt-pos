use escpos::{driver::NativeUsbDriver, printer::Printer};
use nusb::{list_devices, MaybeFuture};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::CommandResult;

pub trait NewPrinter: Send {}
pub type PrinterWrapper = Option<Box<dyn NewPrinter>>;

struct DummyPrinter {}
struct UsbPrinter {
    printer: Printer<NativeUsbDriver>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvailablePrinter {
    vendor_id: u16,
    product_id: u16,
    class: u8,
    subclass: u8,
    manufacturer: Option<String>,
    product: Option<String>,
}

type NewPrinterState = Arc<Mutex<PrinterWrapper>>;

impl NewPrinter for DummyPrinter {}

impl NewPrinter for UsbPrinter {}

pub fn setup_new_printer() -> PrinterWrapper {
    PrinterWrapper::default()
}

pub fn list_available_printers() -> CommandResult<Vec<AvailablePrinter>> {
    let device_list = list_devices().wait()?;

    let mut printers = vec![];
    for device_info in device_list {
        printers.push(AvailablePrinter {
            vendor_id: device_info.vendor_id(),
            product_id: device_info.product_id(),

            class: device_info.class(),
            subclass: device_info.subclass(),

            manufacturer: device_info
                .manufacturer_string()
                .and_then(|s| Some(String::from(s))),
            product: device_info
                .product_string()
                .and_then(|s| Some(String::from(s))),
        });
    }

    println!("Printers {:?}", printers);

    Ok(printers)
}
