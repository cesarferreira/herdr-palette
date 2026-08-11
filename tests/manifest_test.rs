use std::fs;

#[test]
fn declares_the_required_herdr_popup_picker() {
    let manifest =
        fs::read_to_string("herdr-plugin.toml").expect("plugin manifest should be readable");

    assert!(manifest.contains("id = \"cesarferreira.herdr-palette\""));
    assert!(manifest.contains("min_herdr_version = \"0.7.0\""));
    assert!(manifest.contains("[[panes]]\nid = \"picker\""));
    assert!(manifest.contains("placement = \"popup\""));
    assert!(manifest.contains("width = \"80%\""));
    assert!(manifest.contains("command = [\"bin/herdr-palette\"]"));
}
