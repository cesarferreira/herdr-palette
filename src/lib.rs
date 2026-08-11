pub mod catalog;
pub mod command;
pub mod config;
pub mod search;
pub mod ui;

pub use catalog::{Category, Invocation, PaletteItem};
pub use command::{execute, ExecutionError};
pub use search::rank;
pub use ui::{run_ui, UiError};

#[derive(Debug)]
pub struct AppError(pub String);

pub fn run() -> Result<(), AppError> {
    let items = config::load_effective_items(None).map_err(|error| AppError(error.to_string()))?;
    let herdr_bin = std::env::var_os("HERDR_BIN_PATH").unwrap_or_else(|| "herdr".into());
    let mut message = None;

    loop {
        let selected = ui::run_ui_with_message(items.clone(), message.as_deref())
            .map_err(|error| AppError(error.to_string()))?;
        let Some(item) = selected else {
            return Ok(());
        };

        if item.invocation == Invocation::DocumentationOnly {
            message = Some("This action is available through its displayed shortcut.".into());
            continue;
        }

        match command::execute(&item, herdr_bin.as_os_str()) {
            Ok(()) => return Ok(()),
            Err(error) => message = Some(error.message),
        }
    }
}
