use escpos::{
    driver::{FileDriver, NativeUsbDriver, NetworkDriver},
    printer::Printer,
    utils::{Encoder, JustifyMode, PageCode, Protocol},
};
use log::info;
use nusb::{list_devices, MaybeFuture};
use serde::{Deserialize, Serialize};
use std::{path::Path, time::Duration};
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

    delegate_method!(page_code, code: PageCode);
    delegate_method!(custom, cmd: &[u8]);
    delegate_method!(print);

    pub fn open_cash_drawer(&mut self) -> CommandResult<()> {
        // Full ESC/POS cash drawer command: ESC p m t1 t2
        // The escpos crate's cash_drawer() omits t1/t2, leaving the printer
        // waiting for them and causing the command to fire only every other call.
        self.custom(&[0x1B, 0x70, 0x00, 0x19, 0xFA])?;
        self.print()
    }
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

/// Configuration needed to reconnect to a network printer
#[derive(Debug, Clone)]
pub struct NetworkPrinterConfig {
    pub ip: String,
    pub port: u16,
}

/// Printer wrapper with optional reconnection support.
///
/// Network printers store their IP/port and re-establish a fresh TCP connection
/// before every print job. On Windows, idle TCP connections are silently dropped
/// by the OS and intermediate network devices, so reusing a stored connection is
/// unreliable. Creating a fresh connection per job (< 1 ms on LAN) avoids this
/// entirely without noticeable latency.
pub struct ManagedPrinter {
    pub wrapper: PrinterWrapper,
    pub network_config: Option<NetworkPrinterConfig>,
}

impl ManagedPrinter {
    pub fn file(wrapper: PrinterWrapper) -> Self {
        Self {
            wrapper,
            network_config: None,
        }
    }

    pub fn usb(wrapper: PrinterWrapper) -> Self {
        Self {
            wrapper,
            network_config: None,
        }
    }

    pub fn network(ip: String, port: u16) -> CommandResult<Self> {
        let wrapper = create_network_printer(ip.clone(), port)?;
        Ok(Self {
            wrapper,
            network_config: Some(NetworkPrinterConfig { ip, port }),
        })
    }

    /// For network printers, drop the existing (possibly stale) TCP connection
    /// and open a fresh one. No-op for file/USB printers.
    pub fn refresh_connection(&mut self) -> CommandResult<()> {
        if let Some(cfg) = &self.network_config {
            self.wrapper = create_network_printer(cfg.ip.clone(), cfg.port)?;
        }
        Ok(())
    }
}

/// Thread-safe printer state for use in Tauri
pub type NewPrinterState = Mutex<Option<ManagedPrinter>>;

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
    let driver = NetworkDriver::open(&ip_address, port, Some(Duration::from_secs(5)))?;
    let protocol = Protocol::new(Encoder::new(encoding_rs::UTF_8));
    let printer = Printer::new(driver, protocol, None);
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
#[allow(dead_code)]
pub fn list_available_printers() -> CommandResult<Vec<AvailablePrinter>> {
    const USB_CLASS_PRINTER: u8 = 7;

    let all_printers = list_available_usb_devices()?;

    Ok(all_printers
        .into_iter()
        .filter(|p| p.class == USB_CLASS_PRINTER)
        .collect())
}
