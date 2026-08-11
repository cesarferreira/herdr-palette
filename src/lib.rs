pub mod catalog;
pub mod config;

pub use catalog::{Category, Invocation, PaletteItem};

#[derive(Debug)]
pub struct AppError(pub String);

pub fn run() -> Result<(), AppError> {
    Ok(())
}
