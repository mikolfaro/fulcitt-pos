use serde::Serialize;

#[derive(thiserror::Error, Debug, Serialize)]
#[serde(tag = "type", content = "error")]
pub(crate) enum CommandError {
    #[error("Database error {0}")]
    Database(String),

    #[error("Input error {0}")]
    Input(String),

    #[error("Concurrency resource access error")]
    MutexPoison,

    #[error("Printer error {0}")]
    Printer(String),
}

pub(crate) type CommandResult<T> = std::result::Result<T, CommandError>;

impl <D> From<std::sync::PoisonError<D>> for CommandError {
    fn from(err: std::sync::PoisonError<D>) -> Self {
        log::error!("Mutex poisoning error occurred {:?}", err);

        CommandError::MutexPoison
    }
}

impl From<escpos::errors::PrinterError> for CommandError {
    fn from(err: escpos::errors::PrinterError) -> Self {
        log::error!("Printer error occurred {:?}", err);

        CommandError::Printer(err.to_string())
    }
}

impl From<sqlx::Error> for CommandError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("SQLx database error occurred {:?}", err);

        CommandError::Database(err.to_string())
    }
}
