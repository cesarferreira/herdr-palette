# Remove obsolete Rust implementation

## Scope

Herdr Palette is now a Bun and TypeScript plugin. Remove the unreferenced Rust
crate, its Rust source, and Rust-only tests from the repository. Retain the
historical Rust design and migration documents as project history.

## Changes

- Delete Cargo package metadata and lockfile.
- Delete Rust source files under `src/`.
- Delete Rust integration tests and the release-install shell test that depend
  on the removed binary.
- Leave the Bun manifest, TypeScript implementation, tests, README, and
  historical documentation unchanged.

## Verification

Run the Bun typecheck and test suite. Confirm no Cargo files, Rust source, or
Rust tests remain tracked.
