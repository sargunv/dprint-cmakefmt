# Project Checklist

Concrete steps to get `dprint-cmakefmt` to a fully working implementation.
Documentation, release, and publishing work is intentionally out of scope here.

## Layer 1: Build Foundation

- [x] Confirm `cmakefmt-rust` compiles cleanly for `wasm32-unknown-unknown` with
      `default-features = false`.
- [x] Keep crate output as raw dprint Wasm: `cdylib` targeting
      `wasm32-unknown-unknown`.
- [x] Ensure release Wasm builds via `mise run build-wasm`.
- [x] Ensure release Wasm validates via `mise run validate-wasm`.
- [x] Inspect Wasm imports and confirm they are only expected dprint/core
      imports.

## Layer 2: dprint Plugin ABI

- [x] Implement `SyncPluginHandler`.
- [x] Provide stable `PluginInfo`: name, version, `config_key = "cmakefmt"`, and
      placeholder URLs as needed.
- [x] Wire `generate_plugin_code!`.
- [x] Return license text from bundled license content.
- [x] Implement `check_config_updates` as a no-op initially.
- [x] Verify exported ABI includes schema-4 dprint functions.

## Layer 3: File Matching

- [x] Match `*.cmake` using `FileMatchingInfo.file_extensions`.
- [x] Match `CMakeLists.txt` using `FileMatchingInfo.file_names`.
- [x] Add tests proving both paths are selected by dprint/plugin config
      resolution.

## Layer 4: Configuration Resolution

- [x] Define serializable resolved config struct.
- [x] Map dprint global `lineWidth` to cmakefmt line width.
- [x] Map dprint global `indentWidth` to cmakefmt tab size or indent behavior
      where supported.
- [x] Map dprint global `useTabs` to cmakefmt indentation behavior where
      supported.
- [x] Add plugin-specific config keys for the first useful cmakefmt options.
- [x] Reject or diagnose unknown plugin config keys.
- [x] Keep all config resolution in memory.
- [x] Avoid `Config::for_file`, `Config::from_file`, `Config::from_files`, and
      config source discovery APIs.

## Layer 5: Formatter Integration

- [x] Convert dprint `file_bytes` to UTF-8 source text.
- [x] Convert resolved dprint config into `cmakefmt::Config`.
- [x] Call `cmakefmt::format_source(source, &config)`.
- [x] Return `Ok(None)` when formatted output is identical.
- [x] Return `Ok(Some(bytes))` when formatting changes text.
- [x] Return useful dprint formatting errors for invalid input or cmakefmt
      failures.
- [x] Respect cancellation token where practical before and after expensive
      formatting work.

## Layer 6: Behavior Fixtures

- [x] Add fixture for basic `CMakeLists.txt` formatting.
- [x] Add fixture for `*.cmake` formatting.
- [x] Add fixture for no-change formatting.
- [x] Add fixture for line width behavior.
- [x] Add fixture for indentation behavior.
- [x] Add fixture for unknown config diagnostics.
- [x] Add fixture proving no filesystem config discovery is required.

## Layer 7: End-to-End dprint Validation

- [x] Build release Wasm.
- [x] Create test dprint config pointing at local Wasm artifact.
- [x] Run `dprint check` against CMake fixtures.
- [x] Run `dprint fmt` against CMake fixtures.
- [x] Verify dprint reports no change after formatting.
- [x] Verify plugin works from a clean checkout with only declared toolchain
      tasks.

## Layer 8: Hardening

- [x] Add regression test for non-UTF-8 input behavior.
- [x] Add regression test for malformed CMake input behavior.
- [x] Confirm large-file formatting does not panic.
- [x] Confirm repeated formatting calls reuse/release dprint config safely.
- [x] Confirm range formatting behavior is either correct or explicitly
      unsupported/falls back safely.
- [x] Run `mise run check` as the final gate.
