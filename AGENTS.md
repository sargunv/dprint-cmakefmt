# dprint-cmakefmt

Rust dprint plugin for formatting CMake files with cmakefmt.

## Project Map

- `src/lib.rs`: dprint Wasm plugin wrapper around the cmakefmt library API.
- `mise.toml`: pinned toolchain and common tasks.
- `hk.pkl`: check/fix orchestration.
- `dprint.jsonc`: repository formatting configuration.
- `scripts/fetch_cmakefmt.sh`: fetch and patch cmakefmt-rust source.
- `support/patches/`: local patches applied to fetched third-party source.

## Dev Tool Commands

- `mise run check`: run repository checks through hk.
- `mise run fix`: run auto-fixes through hk.
- `mise run build`: build the release Wasm plugin artifact.
- `mise run validate`: build and validate the release Wasm artifact.

Use `mise tasks ls --all` for the full task list.

## Project Invariants

- Runtime formatting must stay in memory.
- Do not use cmakefmt filesystem config discovery APIs in the plugin.
- The release artifact targets `wasm32-unknown-unknown`.
- The plugin should be a raw dprint Wasm plugin module, not a browser Wasm
  package.
