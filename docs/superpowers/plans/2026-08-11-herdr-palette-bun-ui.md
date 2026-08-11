# Herdr Palette Bun UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the Rust command-palette UI with a polished Bun/OpenTUI popup modeled on the provided reference.

**Architecture:** TypeScript owns catalog loading, fuzzy filtering, interaction, rendering, and safe Herdr argv invocation. The Herdr manifest launches Bun inside the existing popup; the plugin reads Herdr config but owns no configuration or state.

**Tech Stack:** Bun, TypeScript, OpenTUI, TOML parsing, Vitest.

## Global Constraints

- Bun is a required runtime dependency.
- Preserve effective Herdr shortcut display and safe `HERDR_BIN_PATH` argv execution.
- No plugin-owned configuration or state.
- Documentation-only actions must never spawn a command.
- The UI must use compact grouped rows, aligned shortcut badges, a yellow selection accent, and a fixed hint footer.

---

### Task 1: Replace packaging and manifest entrypoint

**Files:**
- Create: `package.json`
- Create: `tsconfig.json`
- Modify: `herdr-plugin.toml`
- Modify: `scripts/install-release.sh`
- Modify: `README.md`
- Delete: `Cargo.toml`, `Cargo.lock`

**Interfaces:**
- Produces: `bun run src/main.ts` as the plugin entrypoint.
- Produces: manifest build command `["bun", "install", "--production"]`.

- [ ] **Step 1: Write a manifest/package test** that asserts Bun install and Bun entrypoint.
- [ ] **Step 2: Run it and confirm failure** with `bun test`.
- [ ] **Step 3: Implement package metadata and manifest** with OpenTUI, TOML, and test dependencies.
- [ ] **Step 4: Replace README install requirements** with Bun and local `herdr plugin link .` directions.
- [ ] **Step 5: Run `bun test && git diff --check` and commit** `feat: migrate palette package to Bun`.

### Task 2: Implement command data, config resolution, and safe execution

**Files:**
- Create: `src/catalog.ts`
- Create: `src/config.ts`
- Create: `src/execute.ts`
- Create: `src/types.ts`
- Test: `test/catalog.test.ts`, `test/config.test.ts`, `test/execute.test.ts`

**Interfaces:**
- Produces: `loadPaletteItems(configPath?: string): PaletteItem[]`.
- Produces: `execute(item: PaletteItem, herdrBin?: string): Promise<ExecutionResult>`.
- Produces: `PaletteItem` with `title`, `category`, `icon`, `shortcuts`, and `invocation`.

- [ ] **Step 1: Write failing tests** for default/remapped shortcuts, documentation-only items, and fake-Herdr argv.
- [ ] **Step 2: Implement typed catalog/config/execution modules** that preserve current documented mappings.
- [ ] **Step 3: Run `bun test` and commit** `feat: port Herdr palette command model`.

### Task 3: Build the OpenTUI command palette

**Files:**
- Create: `src/palette.ts`
- Create: `src/theme.ts`
- Create: `src/main.ts`
- Test: `test/palette.test.ts`

**Interfaces:**
- Produces: `createPalette(items: PaletteItem[]): PaletteController`.
- Produces: `PaletteController.filter(query)`, `move(delta)`, and `select()`.

- [ ] **Step 1: Write failing controller tests** for fuzzy filtering, grouped ordering, selection movement, and documentation-only selection.
- [ ] **Step 2: Implement the visual layout**: indigo surface, title/search header, category labels, icon/title/shortcut rows, yellow selection bar, and fixed footer.
- [ ] **Step 3: Wire keyboard input** for text, Backspace, Ctrl-U, arrows/Ctrl-P/Ctrl-N, Enter, and Escape; restore terminal state on exit.
- [ ] **Step 4: Run `bun test`, manually run the palette, and commit** `feat: add OpenTUI command palette`.

### Task 4: Remove Rust implementation and verify end-to-end packaging

**Files:**
- Delete: `src/*.rs`, `tests/*.rs`
- Modify: `.github/workflows/ci.yml`
- Modify: `.github/workflows/release.yml`
- Modify: `.gitignore`

- [ ] **Step 1: Update CI** to install Bun, run formatting/type-check/tests, and remove Rust/release-binary jobs.
- [ ] **Step 2: Delete obsolete Rust source/tests and binary installer behavior.**
- [ ] **Step 3: Run `bun install --frozen-lockfile && bun run typecheck && bun test && git diff --check`.**
- [ ] **Step 4: Link and smoke-test with Herdr if available; otherwise record that exact environment limitation.**
- [ ] **Step 5: Commit** `chore: complete Bun palette migration`.

