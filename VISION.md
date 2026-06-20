# dprint-cmakefmt Vision

## Purpose

Build a dprint plugin that formats CMake files by wrapping
[`cmakefmt/cmakefmt`](https://github.com/cmakefmt/cmakefmt). The plugin should
give dprint users first-class formatting for `CMakeLists.txt` and `*.cmake`
without shelling out to a host binary.

## Product Shape

`dprint-cmakefmt` should be a small Rust crate that compiles to a raw
`wasm32-unknown-unknown` module implementing the dprint plugin ABI. It is not a
browser Wasm package and should not require JavaScript glue, filesystem access,
or host imports at runtime.

The plugin should:

- Match `CMakeLists.txt` and `*.cmake` by default.
- Receive source text and resolved dprint configuration from the host.
- Convert dprint globals such as `lineWidth`, `indentWidth`, and `useTabs` into
  cmakefmt configuration where practical.
- Allow plugin-specific configuration to express cmakefmt options from memory,
  including a path toward custom command specs.
- Return formatted text or no change through the dprint ABI.

## Architecture

The core dependency is the `cmakefmt-rust` package, consumed as the `cmakefmt`
library crate with default features disabled:

```toml
cmakefmt = { package = "cmakefmt-rust", version = "1.6.0", default-features = false }
```

The main library entry point is:

```rust
cmakefmt::format_source(source: &str, config: &cmakefmt::Config)
```

This is the right embedding model for dprint: source string in, formatted string
out, explicit configuration, and no runtime file discovery. cmakefmt's built-in
command registry is generated at build time and embedded into the library, which
is compatible with a Wasm plugin artifact.

## Runtime Boundaries

The plugin must not call cmakefmt file discovery APIs such as
`Config::for_file`, `Config::from_file`, `Config::from_files`, or
`Config::config_sources_for`. dprint is responsible for discovery and should
supply all configuration in memory.

The final `.wasm` artifact should:

- Target `wasm32-unknown-unknown`.
- Export the dprint plugin ABI.
- Avoid browser `wasm-bindgen` entry points.
- Require no runtime filesystem access.
- Have no unexpected imports beyond the dprint ABI expected by `dprint-core`.

## Upstream Risk

cmakefmt already builds for a browser playground, but that support is currently
browser-oriented. The upstream crate enables `wasm-bindgen` and exposes
`src/wasm.rs` for every `wasm32` target, which may be the main friction for a
pure dprint plugin.

The clean upstream shape is to gate browser Wasm behind an explicit feature, for
example `browser-wasm`, so embedding users can compile the pure formatter
library to `wasm32-unknown-unknown` without JavaScript ABI baggage.

Other likely compile-test targets are `serde_yaml_ng` and any public file APIs
that may compile into pure Wasm even when unused. The expected outcome is that
these are manageable packaging issues, not architectural blockers.

## Verification

Development is not complete until the project can verify:

- `cargo check --target wasm32-unknown-unknown`
- `cargo build --release --target wasm32-unknown-unknown`
- The Wasm artifact validates with `wasm-tools validate`.
- Wasm imports are understood and limited to the dprint ABI.
- A smoke fixture formats CMake text in memory.
- Browser `wasm-bindgen` glue is not linked into the plugin artifact.

## Milestones

1. Bootstrap the Rust crate, dprint plugin ABI wrapper, and local toolchain.
2. Compile-test `cmakefmt-rust` with default features disabled for
   `wasm32-unknown-unknown`.
3. Patch or upstream-gate cmakefmt browser Wasm support if needed.
4. Add a real dprint plugin schema and richer configuration mapping.
5. Add artifact inspection and fixture-based plugin tests.
6. Publish release artifacts through the dprint plugin distribution flow.
