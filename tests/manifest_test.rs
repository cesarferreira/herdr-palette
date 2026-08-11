use std::fs;
use toml::Value;

#[test]
fn declares_the_required_herdr_popup_picker() {
    let manifest =
        fs::read_to_string("herdr-plugin.toml").expect("plugin manifest should be readable");
    let manifest: Value = manifest
        .parse()
        .expect("plugin manifest should be valid TOML");

    assert_eq!(manifest["id"].as_str(), Some("cesarferreira.herdr-palette"));
    assert_eq!(manifest["min_herdr_version"].as_str(), Some("0.7.0"));

    let picker = manifest["panes"]
        .as_array()
        .and_then(|panes| {
            panes
                .iter()
                .find(|pane| pane["id"].as_str() == Some("picker"))
        })
        .expect("picker pane should be declared");

    assert_eq!(picker["placement"].as_str(), Some("popup"));
    assert_eq!(picker["width"].as_str(), Some("80%"));
    assert_eq!(
        picker["command"]
            .as_array()
            .expect("picker command should be an array")
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>(),
        ["./target/release/herdr-palette"]
    );
}
