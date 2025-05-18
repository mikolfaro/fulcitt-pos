use serde::Serialize;

#[derive(thiserror::Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub(crate) enum CommandError {
    #[error("Database error {0}")]
    Database(String),

    #[error("Serialization error {0}")]
    Serde(String),

    #[error("IO error {0}")]
    Io(String),

    #[error("Input error {0}")]
    InvalidInput(String),

    #[error("Invalid printer device")]
    InvalidPrinterDevice,

    #[error("Concurrency resource access error")]
    MutexPoison,

    #[error("Printer error {0}")]
    Printer(String),

    #[error("Failed to load settings")]
    LoadSettings,

    #[error("Failed to save settings")]
    StoreSettings,
}

pub(crate) type CommandResult<T> = std::result::Result<T, CommandError>;

impl From<escpos::errors::PrinterError> for CommandError {
    fn from(err: escpos::errors::PrinterError) -> Self {
        log::error!("Printer error occurred {:?}", err);

        CommandError::Printer(err.to_string())
    }
}

impl From<serde_json::Error> for CommandError {
    fn from(err: serde_json::Error) -> Self {
        log::error!("Serializtion error occurred {:?}", err);

        CommandError::Serde(err.to_string())
    }
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        log::error!("IO error occurred {:?}", err);

        CommandError::Io(err.to_string())
    }
}

impl<D> From<std::sync::PoisonError<D>> for CommandError {
    fn from(err: std::sync::PoisonError<D>) -> Self {
        log::error!("Mutex poisoning error occurred {:?}", err);

        CommandError::MutexPoison
    }
}

impl From<sqlx::Error> for CommandError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("SQLx database error occurred {:?}", err);

        CommandError::Database(err.to_string())
    }
}
