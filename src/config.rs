use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
};

use toml::Value;

use crate::{
    catalog::{default_items, Category},
    Invocation, PaletteItem,
};

#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for ConfigError {}

pub fn load_effective_items(config_path: Option<&Path>) -> Result<Vec<PaletteItem>, ConfigError> {
    let Some(path) = config_path.map(PathBuf::from).or_else(discover_config_path) else {
        return Ok(default_items());
    };
    if !path.exists() {
        return Ok(default_items());
    }

    let source = fs::read_to_string(&path)
        .map_err(|error| ConfigError(format!("could not read {}: {error}", path.display())))?;
    let document = source
        .parse::<Value>()
        .map_err(|error| ConfigError(format!("could not parse {}: {error}", path.display())))?;
    let keys = document.get("keys").and_then(Value::as_table);
    let prefix = keys
        .and_then(|keys| keys.get("prefix"))
        .and_then(Value::as_str)
        .unwrap_or("ctrl+b");
    let mut items = default_items();
    for item in &mut items {
        item.shortcuts = item
            .shortcuts
            .iter()
            .map(|shortcut| expand_prefix(shortcut, prefix))
            .collect();
    }

    if let Some(keys) = keys {
        for item in &mut items {
            if let Some(value) = keys.get(&item.id) {
                if let Some(shortcuts) = shortcuts(value, prefix) {
                    item.shortcuts = shortcuts;
                }
            }
        }
        if let Some(commands) = keys.get("command").and_then(Value::as_array) {
            items.extend(
                commands
                    .iter()
                    .enumerate()
                    .filter_map(|(index, command)| custom_command(index, command, prefix)),
            );
        }
    }
    Ok(items)
}

fn discover_config_path() -> Option<PathBuf> {
    if let Some(path) = env::var_os("HERDR_CONFIG_PATH") {
        return Some(PathBuf::from(path));
    }
    let mut candidates = Vec::new();
    if let Some(xdg) = env::var_os("XDG_CONFIG_HOME") {
        candidates.push(PathBuf::from(xdg).join("herdr/config.toml"));
    }
    if let Some(home) = env::var_os("HOME") {
        candidates.push(PathBuf::from(&home).join(".config/herdr/config.toml"));
        #[cfg(target_os = "macos")]
        candidates.push(PathBuf::from(home).join("Library/Application Support/herdr/config.toml"));
    }
    candidates.into_iter().find(|path| path.exists())
}

fn shortcuts(value: &Value, prefix: &str) -> Option<Vec<String>> {
    let values = match value {
        Value::String(value) => vec![value.as_str()],
        Value::Array(values) => values
            .iter()
            .map(Value::as_str)
            .collect::<Option<Vec<_>>>()?,
        _ => return None,
    };
    Some(
        values
            .iter()
            .map(|shortcut| expand_prefix(shortcut, prefix))
            .collect(),
    )
}

fn expand_prefix(shortcut: &str, prefix: &str) -> String {
    shortcut
        .split('+')
        .map(|part| if part == "prefix" { prefix } else { part })
        .collect::<Vec<_>>()
        .join("+")
}

fn custom_command(index: usize, command: &Value, prefix: &str) -> Option<PaletteItem> {
    let command = command.as_table()?;
    let key = command.get("key")?.as_str()?;
    let command_text = command
        .get("command")
        .and_then(Value::as_str)
        .unwrap_or("Custom command");
    let description = command
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or(command_text);
    Some(PaletteItem {
        id: format!("custom_command_{index}"),
        title: description.into(),
        category: Category::Custom,
        description: description.into(),
        aliases: vec![command_text.into()],
        shortcuts: vec![expand_prefix(key, prefix)],
        invocation: Invocation::DocumentationOnly,
    })
}
