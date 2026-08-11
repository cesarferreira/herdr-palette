# Herdr Palette Bun UI redesign

## Goal

Replace the Rust terminal renderer with a Bun and TypeScript OpenTUI command palette that matches the dense, elegant visual language of the supplied reference while retaining Herdr-aware command discovery and execution.

## Architecture

The plugin becomes a Bun package. Its TypeScript entrypoint loads the effective Herdr keymap and command catalog, renders the popup with OpenTUI, and invokes Herdr through `HERDR_BIN_PATH`. The Herdr manifest launches the Bun entrypoint in the existing popup pane.

The plugin reads no plugin-owned settings or state. It reads Herdr configuration solely to display the user's actual bindings. Bun is an explicit runtime dependency.

## Visual system

The palette uses a dark indigo background, compact rows, and a bright yellow accent. The header contains the palette title, focused search field, and an Escape hint. Commands are grouped into named categories with accent headings. Every row includes an icon, title, optional description, and a right-aligned effective shortcut.

The selected row receives a subtly lighter background, a bright left accent bar, and high-contrast title text. The fixed footer shows Enter, navigation, and result-count hints.

## Interaction

Typing filters the catalog immediately across titles, aliases, descriptions, and shortcut labels. Up/Down and Ctrl-P/Ctrl-N move the selection, Enter invokes executable actions, and Escape closes. Documentation-only actions remain discoverable and explain which shortcut to use rather than attempting an unsupported operation.

## Packaging

`package.json` declares Bun and OpenTUI dependencies. Herdr's build command runs `bun install --production`, and its popup entrypoint executes the installed TypeScript application with Bun. The README states Bun as a prerequisite and replaces binary-download instructions.

## Verification

Tests cover effective-keymap loading, catalog construction, fuzzy filtering, item selection, command invocation, and visual model construction. A manual smoke check links the plugin, opens the popup, confirms category/shortcut alignment, filters a command, invokes a supported action, and dismisses a documentation-only action.

