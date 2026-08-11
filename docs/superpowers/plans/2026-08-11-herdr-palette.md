# Herdr Palette Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a zero-configuration popup palette that discovers and invokes Herdr actions while showing the user’s effective shortcuts.

**Architecture:** A Rust terminal application runs as a Herdr popup pane. It loads a static, versioned catalog of API-backed Herdr actions and merges the user’s `config.toml` key bindings, including `[[keys.command]]`. The UI fuzzy-filters the catalog and delegates execution to `HERDR_BIN_PATH` using argv arrays.

**Tech Stack:** Rust stable, ratatui/crossterm for terminal rendering and input, serde/toml for configuration, nucleo-matcher for fuzzy ranking, cargo test/clippy/fmt, GitHub Actions.

## Global Constraints

- The plugin owns no user configuration or durable state files.
- Read the standard Herdr config and use `HERDR_BIN_PATH`, falling back to `herdr` only for local development.
- Use argv process spawning; never evaluate arbitrary shell strings.
- Ship Linux and macOS binaries; the installed popup has no Node, Bun, fzf, or package-manager runtime dependency.
- Use only documented Herdr CLI/socket operations; never simulate a Herdr prefix key.
- Actions that require interactive text (rename, labels, branch names) are out of the initial invokable catalog unless the palette collects that text itself.
- User-defined `[[keys.command]]` entries are shown as discoverable shortcut documentation, but are not re-executed by the plugin because their command semantics are owned by Herdr.
- Require Herdr `0.7.0` or newer.

---

## File structure

- `Cargo.toml`: Rust package metadata and dependencies.
- `src/lib.rs`: public module boundary and reusable application entry point.
- `src/catalog.rs`: immutable Herdr action catalog and invocation types.
- `src/config.rs`: Herdr config discovery and keybinding/custom-command parsing.
- `src/command.rs`: safe argv construction and child-process execution.
- `src/search.rs`: fuzzy ranking and filtering.
- `src/ui.rs`: terminal event loop, rendering, selection, and error state.
- `src/main.rs`: thin production executable wrapper.
- `tests/config_test.rs`: config and effective-shortcut tests.
- `tests/catalog_test.rs`: catalog and invocation tests.
- `tests/search_test.rs`: fuzzy-search tests.
- `tests/execution_test.rs`: fake-Herdr process integration tests.
- `herdr-plugin.toml`: Herdr package metadata and popup pane declaration.
- `scripts/install-release.sh`: download the correct release binary at Herdr plugin install time.
- `README.md`: installation, binding, compatibility, and manual smoke test.
- `.github/workflows/ci.yml`: format, lint, and test checks.
- `.github/workflows/release.yml`: tagged Linux/macOS release artifact builds.

### Task 1: Scaffold a testable Rust package and Herdr manifest

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `src/main.rs`
- Create: `herdr-plugin.toml`
- Create: `.gitignore`

**Interfaces:**
- Produces: `herdr_palette::run() -> Result<(), AppError>`.
- Produces: a Herdr `picker` pane entrypoint that runs `bin/herdr-palette` in a popup.

- [ ] **Step 1: Write the manifest and package-level acceptance test**

Create `tests/manifest_test.rs` that reads `herdr-plugin.toml` and asserts these literal fields: `id = "cesarferreira.herdr-palette"`, `min_herdr_version = "0.7.0"`, pane `id = "picker"`, `placement = "popup"`, `width = "80%"`, and command `["bin/herdr-palette"]`.

- [ ] **Step 2: Run the acceptance test to verify it fails**

Run: `cargo test --test manifest_test`

Expected: FAIL because the Cargo package and manifest do not yet exist.

- [ ] **Step 3: Add the minimal package and manifest**

Create a Rust 2021 package named `herdr-palette` with library crate `herdr_palette`. Define:

```rust
pub fn run() -> Result<(), AppError> {
    Ok(())
}

#[derive(Debug)]
pub struct AppError(pub String);
```

Make `src/main.rs` call `herdr_palette::run()` and print a nonzero-error message. Add the manifest:

```toml
id = "cesarferreira.herdr-palette"
name = "Herdr Palette"
version = "0.1.0"
min_herdr_version = "0.7.0"
description = "Fuzzy command palette that teaches effective Herdr shortcuts"
platforms = ["linux", "macos"]

[[build]]
command = ["sh", "scripts/install-release.sh"]

[[panes]]
id = "picker"
title = "Herdr Palette"
placement = "popup"
width = "80%"
height = 20
command = ["bin/herdr-palette"]
```

- [ ] **Step 4: Re-run the package and manifest checks**

Run: `cargo test --test manifest_test && cargo test && cargo fmt --check`

Expected: PASS.

- [ ] **Step 5: Commit the scaffold**

```bash
git add Cargo.toml src herdr-plugin.toml .gitignore tests/manifest_test.rs
git commit -m "feat: scaffold Herdr palette plugin"
```

### Task 2: Implement effective Herdr keymap and documentation catalog

**Files:**
- Create: `src/catalog.rs`
- Create: `src/config.rs`
- Modify: `src/lib.rs`
- Test: `tests/config_test.rs`
- Test: `tests/catalog_test.rs`

**Interfaces:**
- Produces: `pub struct PaletteItem { pub id: String, pub title: String, pub category: Category, pub description: String, pub aliases: Vec<String>, pub shortcuts: Vec<String>, pub invocation: Invocation }`.
- Produces: `pub enum Invocation { Herdr(Vec<String>), DocumentationOnly }`.
- Produces: `pub fn load_effective_items(config_path: Option<&Path>) -> Result<Vec<PaletteItem>, ConfigError>`.
- Consumes: `PaletteItem` in search, UI, and command execution.

- [ ] **Step 1: Write failing keymap and catalog tests**

In `tests/config_test.rs`, create a temporary `config.toml` containing:

```toml
[keys]
prefix = "ctrl+a"
new_tab = ["prefix+c", "ctrl+alt+c"]
zoom = "ctrl+alt+z"

[[keys.command]]
key = "prefix+g"
type = "shell"
command = "git status"
description = "Show Git status"
```

Assert that `new_tab` renders as `["ctrl+a+c", "ctrl+alt+c"]`, `zoom` as `["ctrl+alt+z"]`, and that the custom command is present with `DocumentationOnly`. In `tests/catalog_test.rs`, assert the catalog contains `new_tab` with `Herdr(["tab", "create", "--focus"])`, `split_vertical` with `Herdr(["pane", "split", "--current", "--direction", "right", "--focus"])`, and `focus_pane_left` with `Herdr(["pane", "focus", "--direction", "left", "--current"])`.

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test --test config_test --test catalog_test`

Expected: FAIL because `config` and `catalog` modules do not exist.

- [ ] **Step 3: Implement immutable defaults and config merging**

Add serde and toml dependencies. In `catalog.rs`, define the API-backed catalog for: `new_workspace`, `new_tab`, `previous_tab`, `next_tab`, `focus_pane_left`, `focus_pane_down`, `focus_pane_up`, `focus_pane_right`, `split_vertical`, `split_horizontal`, `swap_pane_left`, `swap_pane_down`, `swap_pane_up`, `swap_pane_right`, `zoom`, and `close_pane`. Mark non-API UI shortcuts such as `help`, `settings`, `copy_mode`, `resize_mode`, and `detach` as `DocumentationOnly` with their descriptions.

In `config.rs`, resolve the config path in this order: `HERDR_CONFIG_PATH` when set; `$XDG_CONFIG_HOME/herdr/config.toml`; `$HOME/.config/herdr/config.toml`; on macOS, `$HOME/Library/Application Support/herdr/config.toml`. Missing config yields defaults. Parse a string or array of strings for known `[keys]` members, replace `prefix` tokens using the effective `keys.prefix`, and parse `[[keys.command]]` tables into documentation-only items.

- [ ] **Step 4: Run focused tests**

Run: `cargo test --test config_test --test catalog_test`

Expected: PASS.

- [ ] **Step 5: Commit the catalog layer**

```bash
git add Cargo.toml Cargo.lock src/catalog.rs src/config.rs src/lib.rs tests/config_test.rs tests/catalog_test.rs
git commit -m "feat: resolve Herdr shortcuts into palette items"
```

### Task 3: Implement fuzzy selection and terminal popup UI

**Files:**
- Create: `src/search.rs`
- Create: `src/ui.rs`
- Modify: `src/lib.rs`
- Test: `tests/search_test.rs`

**Interfaces:**
- Produces: `pub fn rank(query: &str, items: &[PaletteItem]) -> Vec<usize>`.
- Produces: `pub fn run_ui(items: Vec<PaletteItem>) -> Result<Option<PaletteItem>, UiError>`.
- Consumes: `PaletteItem` and returns the selected item or `None` on Escape.

- [ ] **Step 1: Write failing fuzzy-rank tests**

In `tests/search_test.rs`, create `New tab`, `Split right`, and `Focus pane left` items. Assert `rank("nt", ...)` puts `New tab` first; `rank("ctrl+a+c", ...)` matches an item by shortcut; `rank("focus left", ...)` matches `Focus pane left`; and an empty query preserves input order.

- [ ] **Step 2: Run the tests to verify they fail**

Run: `cargo test --test search_test`

Expected: FAIL because `search::rank` does not exist.

- [ ] **Step 3: Implement search and UI**

Add `nucleo-matcher`, `ratatui`, and `crossterm`. Rank against a concatenated title, description, aliases, and shortcut string; break equal scores by source catalog order. `run_ui` must enable raw mode and enter the alternate screen, restoring both with a cleanup guard even after an error. Render a title, query field, scrollable result list, selected shortcut badges, and the selected description. Bind printable input, Backspace, Ctrl-U, Up/Down, Ctrl-P/Ctrl-N, Enter, and Escape. Return `Some(item)` on Enter and `None` on Escape.

- [ ] **Step 4: Run unit tests and a manual terminal smoke check**

Run: `cargo test --test search_test && cargo test`

Expected: PASS.

Then run: `cargo run --bin herdr-palette`

Expected: a usable alternate-screen palette; Escape restores the original terminal exactly.

- [ ] **Step 5: Commit the picker UI**

```bash
git add Cargo.toml Cargo.lock src/search.rs src/ui.rs src/lib.rs tests/search_test.rs
git commit -m "feat: add fuzzy popup palette UI"
```

### Task 4: Execute supported actions safely and surface errors

**Files:**
- Create: `src/command.rs`
- Modify: `src/lib.rs`
- Modify: `src/ui.rs`
- Test: `tests/execution_test.rs`

**Interfaces:**
- Produces: `pub fn execute(item: &PaletteItem, herdr_bin: &OsStr) -> Result<(), ExecutionError>`.
- Consumes: `Invocation::Herdr`; rejects `DocumentationOnly` without spawning a child process.
- Produces: `ExecutionError { message: String }` suitable for UI display.

- [ ] **Step 1: Write failing process tests**

In `tests/execution_test.rs`, create an executable temporary fake Herdr script that writes received argv to a temporary file and exits zero. Set `HERDR_BIN_PATH` to that script. Assert that executing the `new_tab` item writes exactly `tab\ncreate\n--focus\n`. Add a fake script exiting 17 with `cannot create tab` on stderr and assert the returned `ExecutionError.message` contains both `17` and `cannot create tab`. Assert a documentation-only item does not create the argv output file.

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test execution_test`

Expected: FAIL because `command::execute` does not exist.

- [ ] **Step 3: Implement argv execution and UI error state**

Use `std::process::Command` with the supplied executable and the invocation argv. Capture stderr, trim it, and report either stderr or the status code. Select the executable from `HERDR_BIN_PATH`, otherwise `herdr`. When an item is `DocumentationOnly`, keep the popup open and show: `This action is available through its displayed shortcut.` When execution fails, restore the UI and display the error in a modal-like message until Escape or Enter. On success, return from `run()` so Herdr closes the popup.

- [ ] **Step 4: Run all Rust checks**

Run: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`

Expected: PASS.

- [ ] **Step 5: Commit safe execution**

```bash
git add src/command.rs src/lib.rs src/ui.rs tests/execution_test.rs
git commit -m "feat: invoke Herdr actions from palette"
```

### Task 5: Package binary installation, CI, and user documentation

**Files:**
- Create: `scripts/install-release.sh`
- Create: `.github/workflows/ci.yml`
- Create: `.github/workflows/release.yml`
- Create: `README.md`
- Modify: `herdr-plugin.toml`

**Interfaces:**
- Produces: `bin/herdr-palette`, installed by the manifest build command.
- Consumes: GitHub release assets named `herdr-palette-x86_64-unknown-linux-gnu.tar.gz`, `herdr-palette-aarch64-apple-darwin.tar.gz`, and `herdr-palette-x86_64-apple-darwin.tar.gz`.

- [ ] **Step 1: Write install-script acceptance checks**

Create `tests/install_script_test.sh` that stubs `uname` and `curl` through a temporary `PATH`. Assert the script selects `x86_64-unknown-linux-gnu` for Linux x86_64 and `aarch64-apple-darwin` for Darwin arm64. Assert an unknown tuple exits nonzero with `unsupported platform`.

- [ ] **Step 2: Run the script test to verify it fails**

Run: `sh tests/install_script_test.sh`

Expected: FAIL because `scripts/install-release.sh` does not exist.

- [ ] **Step 3: Implement packaging and documentation**

Implement `scripts/install-release.sh` with `set -eu`; determine `uname -s` and `uname -m`; map only the three supported tuples; download the matching tagged release asset via curl; extract it into `bin/`; and `chmod +x bin/herdr-palette`. Use the repository's eventual GitHub release URL with the version read from `herdr-plugin.toml`.

Add CI for `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and the shell acceptance test. Add release CI triggered on `v*` tags that builds the three targets, packages the binary, and uploads matching assets.

Write README sections for: what the palette does; install with `herdr plugin install cesarferreira/herdr-palette`; local link with `cargo build --release && herdr plugin link .`; a direct binding:

```toml
[[keys.command]]
key = "ctrl+space"
type = "plugin_pane"
plugin = "cesarferreira.herdr-palette"
pane = "picker"
description = "Open Herdr Palette"
```

Document that the palette reads `config.toml`, including remaps and custom command bindings, owns no config, and exposes only documented API-backed actions for direct execution.

- [ ] **Step 4: Run complete verification**

Run: `sh tests/install_script_test.sh && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test && git diff --check`

Expected: PASS.

- [ ] **Step 5: Perform a live Herdr smoke test and commit**

Run:

```bash
cargo build --release
herdr plugin link .
herdr plugin list --plugin cesarferreira.herdr-palette
```

Expected: the plugin is registered. In a Herdr session, add the documented binding, reload config, press Ctrl-Space, confirm a remapped shortcut is shown, invoke `New tab`, then press Escape from a reopened palette.

Commit:

```bash
git add scripts .github README.md herdr-plugin.toml tests/install_script_test.sh
git commit -m "docs: package and document Herdr Palette"
```

