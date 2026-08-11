use herdr_palette::{catalog::default_items, Invocation};

#[test]
fn maps_supported_actions_to_documented_herdr_argv() {
    let items = default_items();

    assert_eq!(
        items
            .iter()
            .find(|item| item.id == "new_tab")
            .expect("New tab should be catalogued")
            .invocation,
        Invocation::Herdr(vec!["tab".into(), "create".into(), "--focus".into()])
    );
    assert_eq!(
        items
            .iter()
            .find(|item| item.id == "split_vertical")
            .expect("Split vertical should be catalogued")
            .invocation,
        Invocation::Herdr(vec![
            "pane".into(),
            "split".into(),
            "--current".into(),
            "--direction".into(),
            "right".into(),
            "--focus".into(),
        ])
    );
    assert_eq!(
        items
            .iter()
            .find(|item| item.id == "focus_pane_left")
            .expect("Focus pane left should be catalogued")
            .invocation,
        Invocation::Herdr(vec![
            "pane".into(),
            "focus".into(),
            "--direction".into(),
            "left".into(),
            "--current".into(),
        ])
    );
}
