use escpos::{driver::UsbDriver, printer::Printer};
use std::sync::Arc;
use tokio::sync::Mutex;

pub trait NewPrinter: Send {}
pub type PrinterWrapper = Option<Box<dyn NewPrinter>>;

struct DummyPrinter {}
struct UsbPrinter {
    printer: Printer<UsbDriver>,
}

type NewPrinterState = Arc<Mutex<PrinterWrapper>>;

impl NewPrinter for DummyPrinter {}

impl NewPrinter for UsbPrinter {}

pub fn setup_new_printer() -> PrinterWrapper {
    PrinterWrapper::default()
}
