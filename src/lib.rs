pub mod catalog;
pub mod config;
pub mod search;
pub mod ui;

pub use catalog::{Category, Invocation, PaletteItem};
pub use search::rank;
pub use ui::{run_ui, UiError};

#[derive(Debug)]
pub struct AppError(pub String);

pub fn run() -> Result<(), AppError> {
    let items = config::load_effective_items(None).map_err(|error| AppError(error.to_string()))?;
    run_ui(items).map_err(|error| AppError(error.to_string()))?;
    Ok(())
}
