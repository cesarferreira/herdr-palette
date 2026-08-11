# Herdr Palette design

## Goal

Provide a zero-configuration command palette for Herdr. A direct `ctrl+space`
binding opens a fast popup that fuzzy-finds Herdr's native actions, displays
each action's effective shortcut, and invokes the action directly. The palette
is both a fallback for forgotten shortcuts and a way to learn them over time.

## Scope

Version one covers native Herdr actions: navigation, pane/tab/workspace
management, session actions, and other actions exposed by the supported Herdr
CLI or socket API. It is not a generic shell-command launcher and it does not
own a configuration or state file.

The only user configuration is a normal Herdr keybinding that launches the
plugin pane, documented with `ctrl+space` as the suggested direct chord.

## Architecture

The repository is a Rust project that produces one `herdr-palette` executable
and a root `herdr-plugin.toml` manifest.

The manifest declares a `picker` plugin pane with `placement = "popup"`. Herdr
starts the executable in a session-modal terminal popup. The executable loads
the effective Herdr keymap, builds a command catalog, renders a terminal UI,
and invokes a selected action through the active Herdr binary. It then exits so
Herdr closes the popup.

The plugin is self-contained at runtime: it has no Node, Bun, fzf, or other
runtime dependency. Local development uses `cargo build --release`. CI builds
and releases macOS and Linux artifacts.

## Effective keymap

The executable starts with an in-code versioned map of Herdr's default actions,
descriptions, aliases, and default shortcuts. It locates the platform-standard
Herdr configuration and parses its `[keys]` section. User bindings override
the matching default labels; an action configured with no usable binding is
shown as unbound or omitted where it cannot run.

Displaying the resolved binding, rather than only the defaults, ensures that
the palette teaches a user's actual muscle memory after remapping their keys.

## Catalog and interaction

Catalog entries contain an action id, title, category, description, aliases,
effective shortcut labels, and a typed invocation strategy. The initial
catalog includes all native actions that the plugin can map to an official
Herdr CLI or socket API operation.

The UI supports typing to fuzzy-filter titles, descriptions, aliases, and
shortcut labels; Up/Down and Ctrl-P/Ctrl-N navigation; Enter to select; and
Escape to close. Each selected row visibly includes the full effective
shortcut and a short description.

The program does not simulate key presses. It executes the corresponding
Herdr operation directly, using the current workspace/tab/pane context supplied
by Herdr. This prevents remapping from affecting command correctness.

## Execution and errors

All Herdr calls use `HERDR_BIN_PATH` when supplied, with `herdr` as a fallback
for local development. Invocation uses argv arrays, never a user-provided shell
string. The plugin never executes arbitrary shell input, writes persistent
state, or changes the user's Herdr configuration.

Actions unavailable on the running Herdr version or lacking required context
are disabled with a short explanation. Failures remain visible inside the
popup until dismissed, with the command error rendered in a concise actionable
form. Successful actions close the popup.

## Manifest and installation

`herdr-plugin.toml` includes required metadata, a supported minimum Herdr
version, and the popup pane entrypoint. The README documents local linking via
`herdr plugin link .`, the `ctrl+space` Herdr keybinding, and GitHub
installation. The repository will be published with the `herdr-plugin` topic
so the Herdr marketplace discovers it.

## Verification

Unit tests cover default-plus-user keymap resolution, shortcut rendering,
catalog construction, fuzzy ranking, unavailable-context behavior, and argv
construction. Integration-style tests run against a fake `HERDR_BIN_PATH` to
verify exact calls and errors. A manual smoke test links the plugin to a live
Herdr session, binds `ctrl+space`, opens the popup, checks a remapped shortcut,
and invokes an action.

CI runs formatting, linting, and tests, and creates Linux and macOS release
artifacts.

