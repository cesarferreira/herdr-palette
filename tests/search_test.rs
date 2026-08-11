use herdr_palette::{rank, Category, Invocation, PaletteItem};

fn item(title: &str, description: &str, aliases: &[&str], shortcuts: &[&str]) -> PaletteItem {
    PaletteItem {
        id: title.to_lowercase().replace(' ', "_"),
        title: title.into(),
        category: Category::Pane,
        description: description.into(),
        aliases: aliases.iter().map(|alias| (*alias).into()).collect(),
        shortcuts: shortcuts
            .iter()
            .map(|shortcut| (*shortcut).into())
            .collect(),
        invocation: Invocation::DocumentationOnly,
    }
}

fn items() -> Vec<PaletteItem> {
    vec![
        item("New tab", "Create a tab", &["tab create"], &["ctrl+a+c"]),
        item(
            "Split right",
            "Split the current pane",
            &["pane split"],
            &["ctrl+a+v"],
        ),
        item(
            "Focus pane left",
            "Focus the pane to the left",
            &["pane left"],
            &["ctrl+a+h"],
        ),
    ]
}

#[test]
fn ranks_a_title_fuzzy_match_first() {
    let items = items();

    assert_eq!(rank("nt", &items).first(), Some(&0));
}

#[test]
fn finds_items_by_shortcut() {
    let items = items();

    assert_eq!(rank("ctrl+a+c", &items), vec![0]);
}

#[test]
fn finds_items_by_description_words() {
    let items = items();

    assert_eq!(rank("focus left", &items).first(), Some(&2));
}

#[test]
fn preserves_catalog_order_for_an_empty_query() {
    let items = items();

    assert_eq!(rank("", &items), vec![0, 1, 2]);
}
