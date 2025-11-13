use escpos::{
    driver::{FileDriver, NativeUsbDriver, NetworkDriver},
    printer::Printer,
    utils::{JustifyMode, Protocol},
};
use log::info;
use nusb::{list_devices, MaybeFuture};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::errors::CommandResult;

/// Runtime-switchable printer wrapper supporting both file and USB printers
#[derive(Clone)]
pub enum PrinterWrapper {
    /// File printer for debugging/testing (writes to app data directory)
    File(Printer<FileDriver>),
    /// USB ESC/POS thermal printer
    Usb(Printer<NativeUsbDriver>),
    /// Network ESC/POS rinter
    Network(Printer<NetworkDriver>),
}

/// Macro to delegate method calls to both enum variants
macro_rules! delegate_method {
    // No arguments, no return value
    ($method:ident) => {
        pub fn $method(&mut self) -> CommandResult<()> {
            match self {
                PrinterWrapper::File(p) => {
                    p.$method()?;
                }
                PrinterWrapper::Usb(p) => {
                    p.$method()?;
                }
                PrinterWrapper::Network(p) => {
                    p.$method()?;
                }
            }
            Ok(())
        }
    };
    // With arguments, no return value
    ($method:ident, $($arg:ident: $arg_ty:ty),+) => {
        pub fn $method(&mut self, $($arg: $arg_ty),+) -> CommandResult<()> {
            match self {
                PrinterWrapper::File(p) => {
                    p.$method($($arg),+)?;
                }
                PrinterWrapper::Usb(p) => {
                    p.$method($($arg),+)?;
                }
                PrinterWrapper::Network(p) => {
                    p.$method($($arg),+)?;
                }
            }
            Ok(())
        }
    };
}

/// All methods are delegated to `escpos::printer::Printer`
impl PrinterWrapper {
    delegate_method!(init);

    // delegate_method!(write, text: &str);

    delegate_method!(writeln, text: &str);

    delegate_method!(justify, mode: JustifyMode);

    // delegate_method!(bold, enable: bool);

    delegate_method!(size, width: u8, height: u8);

    delegate_method!(feed);

    // delegate_method!(feeds, lines: u8);

    // delegate_method!(cut);

    delegate_method!(print_cut);

    delegate_method!(reset_size);
}

/// USB device information for printer enumeration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AvailablePrinter {
    pub vendor_id: u16,
    pub product_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

/// Thread-safe printer state for use in Tauri
pub type NewPrinterState = Arc<Mutex<Option<PrinterWrapper>>>;

/// Create a file printer that writes to the specified path
/// Use this for testing/debugging - output will be written to the file
pub fn create_file_printer(file_path: &Path) -> CommandResult<PrinterWrapper> {
    // Ensure the parent directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create the file if it doesn't exist
    if !file_path.exists() {
        std::fs::File::create(file_path)?;
    }

    let driver = FileDriver::open(file_path)?;
    let printer = Printer::new(driver, Protocol::default(), None);
    Ok(PrinterWrapper::File(printer))
}

/// Create a USB printer with the specified vendor and product IDs
pub fn create_usb_printer(vendor_id: u16, product_id: u16) -> CommandResult<PrinterWrapper> {
    info!("Setting up USB printer {:x}:{:x}", vendor_id, product_id);
    let driver = NativeUsbDriver::open(vendor_id, product_id)?;
    let printer = Printer::new(driver, Protocol::default(), None);
    Ok(PrinterWrapper::Usb(printer))
}

/// Create a network printer with the specificed IP address
pub fn create_network_printer(ip_address: String, port: u16) -> CommandResult<PrinterWrapper> {
    info!("Setting up network printer {:?}", ip_address);
    let driver = NetworkDriver::open(&ip_address, port, Some(Duration::from_secs(1)))?;
    let printer = Printer::new(driver, Protocol::default(), None);
    Ok(PrinterWrapper::Network(printer))
}

/// List all available USB devices (optionally filtered by printer class)
///
/// Note: USB printer class is typically 7 (Printer), subclass 1 (Printer)
/// However, some printers may identify as vendor-specific (class 0xFF),
/// so we return all devices and let the frontend/user decide.
pub fn list_available_usb_devices() -> CommandResult<Vec<AvailablePrinter>> {
    let device_list = list_devices().wait()?;

    let printers: Vec<AvailablePrinter> = device_list
        .into_iter()
        .map(|device_info| AvailablePrinter {
            vendor_id: device_info.vendor_id(),
            product_id: device_info.product_id(),
            class: device_info.class(),
            subclass: device_info.subclass(),
            manufacturer: device_info.manufacturer_string().map(String::from),
            product: device_info.product_string().map(String::from),
        })
        .collect();

    Ok(printers)
}

/// List only USB devices that identify as printer class (class 7)
pub fn list_available_printers() -> CommandResult<Vec<AvailablePrinter>> {
    const USB_CLASS_PRINTER: u8 = 7;

    let all_printers = list_available_usb_devices()?;

    Ok(all_printers
        .into_iter()
        .filter(|p| p.class == USB_CLASS_PRINTER)
        .collect())
}
