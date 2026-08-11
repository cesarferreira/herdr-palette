use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use herdr_palette::{config::load_effective_items, Invocation};

#[test]
fn resolves_remapped_shortcuts_and_documents_custom_commands() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after the Unix epoch")
        .as_nanos();
    let directory = std::env::temp_dir().join(format!("herdr-palette-config-{unique}"));
    fs::create_dir_all(&directory).expect("temporary config directory should be created");
    let config_path = directory.join("config.toml");
    fs::write(
        &config_path,
        r#"
[keys]
prefix = "ctrl+a"
new_tab = ["prefix+c", "ctrl+alt+c"]
zoom = "ctrl+alt+z"

[[keys.command]]
key = "prefix+g"
type = "shell"
command = "git status"
description = "Show Git status"
"#,
    )
    .expect("temporary config should be written");

    let items = load_effective_items(Some(&config_path)).expect("config should load");
    fs::remove_dir_all(&directory).expect("temporary config directory should be removed");

    let new_tab = items
        .iter()
        .find(|item| item.id == "new_tab")
        .expect("New tab should be present");
    assert_eq!(new_tab.shortcuts, ["ctrl+a+c", "ctrl+alt+c"]);

    let new_workspace = items
        .iter()
        .find(|item| item.id == "new_workspace")
        .expect("New workspace should be present");
    assert_eq!(new_workspace.shortcuts, ["ctrl+a+shift+n"]);

    let zoom = items
        .iter()
        .find(|item| item.id == "zoom")
        .expect("Zoom should be present");
    assert_eq!(zoom.shortcuts, ["ctrl+alt+z"]);

    let custom_command = items
        .iter()
        .find(|item| item.description == "Show Git status")
        .expect("custom command should be documented");
    assert_eq!(custom_command.shortcuts, ["ctrl+a+g"]);
    assert_eq!(custom_command.invocation, Invocation::DocumentationOnly);
}
