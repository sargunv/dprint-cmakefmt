# Project Checklist

Concrete steps to get `dprint-cmakefmt` to a fully working implementation.
Documentation, release, and publishing work is intentionally out of scope here.

## Layer 1: Build Foundation

- [ ] Confirm `cmakefmt-rust` compiles cleanly for `wasm32-unknown-unknown` with
      `default-features = false`.
- [ ] Keep crate output as raw dprint Wasm: `cdylib` targeting
      `wasm32-unknown-unknown`.
- [ ] Ensure release Wasm builds via `mise run build-wasm`.
- [ ] Ensure release Wasm validates via `mise run validate-wasm`.
- [ ] Inspect Wasm imports and confirm they are only expected dprint/core
      imports.

## Layer 2: dprint Plugin ABI

- [ ] Implement `SyncPluginHandler`.
- [ ] Provide stable `PluginInfo`: name, version, `config_key = "cmakefmt"`, and
      placeholder URLs as needed.
- [ ] Wire `generate_plugin_code!`.
- [ ] Return license text from bundled license content.
- [ ] Implement `check_config_updates` as a no-op initially.
- [ ] Verify exported ABI includes schema-4 dprint functions.

## Layer 3: File Matching

- [ ] Match `*.cmake` using `FileMatchingInfo.file_extensions`.
- [ ] Match `CMakeLists.txt` using `FileMatchingInfo.file_names`.
- [ ] Add tests proving both paths are selected by dprint/plugin config
      resolution.

## Layer 4: Configuration Resolution

- [ ] Define serializable resolved config struct.
- [ ] Map dprint global `lineWidth` to cmakefmt line width.
- [ ] Map dprint global `indentWidth` to cmakefmt tab size or indent behavior
      where supported.
- [ ] Map dprint global `useTabs` to cmakefmt indentation behavior where
      supported.
- [ ] Add plugin-specific config keys for the first useful cmakefmt options.
- [ ] Reject or diagnose unknown plugin config keys.
- [ ] Keep all config resolution in memory.
- [ ] Avoid `Config::for_file`, `Config::from_file`, `Config::from_files`, and
      config source discovery APIs.

## Layer 5: Formatter Integration

- [ ] Convert dprint `file_bytes` to UTF-8 source text.
- [ ] Convert resolved dprint config into `cmakefmt::Config`.
- [ ] Call `cmakefmt::format_source(source, &config)`.
- [ ] Return `Ok(None)` when formatted output is identical.
- [ ] Return `Ok(Some(bytes))` when formatting changes text.
- [ ] Return useful dprint formatting errors for invalid input or cmakefmt
      failures.
- [ ] Respect cancellation token where practical before and after expensive
      formatting work.

## Layer 6: Behavior Fixtures

- [ ] Add fixture for basic `CMakeLists.txt` formatting.
- [ ] Add fixture for `*.cmake` formatting.
- [ ] Add fixture for no-change formatting.
- [ ] Add fixture for line width behavior.
- [ ] Add fixture for indentation behavior.
- [ ] Add fixture for unknown config diagnostics.
- [ ] Add fixture proving no filesystem config discovery is required.

## Layer 7: End-to-End dprint Validation

- [ ] Build release Wasm.
- [ ] Create test dprint config pointing at local Wasm artifact.
- [ ] Run `dprint check` against CMake fixtures.
- [ ] Run `dprint fmt` against CMake fixtures.
- [ ] Verify dprint reports no change after formatting.
- [ ] Verify plugin works from a clean checkout with only declared toolchain
      tasks.

## Layer 8: Hardening

- [ ] Add regression test for non-UTF-8 input behavior.
- [ ] Add regression test for malformed CMake input behavior.
- [ ] Confirm large-file formatting does not panic.
- [ ] Confirm repeated formatting calls reuse/release dprint config safely.
- [ ] Confirm range formatting behavior is either correct or explicitly
      unsupported/falls back safely.
- [ ] Run `mise run check` as the final gate.
