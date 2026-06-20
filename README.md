# dprint-cmakefmt

This is a [dprint](https://dprint.dev) Wasm plugin for formatting CMake files
with [`cmakefmt`](https://github.com/cmakefmt/cmakefmt). It can be used to
format `CMakeLists.txt`, `CMakeLists.txt.in`, and `*.cmake` files with dprint.

## Setup

Add the latest plugin release to your `dprint.json`:

```sh
dprint add sargunv/dprint-cmakefmt
```

Then run dprint normally:

```sh
dprint fmt
```

The plugin formats `CMakeLists.txt`, `CMakeLists.txt.in`, and files with the
`.cmake` extension.

## Configure

cmakefmt options go under the `cmakefmt` config key. Use camelCase option names
represented as JSON:

```jsonc
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": ["./target/wasm32-unknown-unknown/release/dprint_cmakefmt.wasm"],
  "cmakefmt": {
    "lineWidth": 100,
    "indentWidth": 2,
    "commandCase": "lower",
    "keywordCase": "upper",
    "dangleParens": false
  }
}
```

dprint global options are also mapped when the matching cmakefmt option is not
set explicitly:

| dprint option | cmakefmt option |
| ------------- | --------------- |
| `lineWidth`   | `lineWidth`     |
| `indentWidth` | `indentWidth`   |
| `useTabs`     | `useTabs`       |
| `newLineKind` | `newLineKind`   |

`cmakefmt` wins over the global option when both are present.

## Limitations

This plugin does not read cmakefmt config files. Put formatter options in dprint
config instead of relying on `.cmakefmt.yaml`, `.cmakefmt.yml`, or
`.cmakefmt.toml`.

Range formatting is not currently supported.

## Patch

We patch `cmakefmt-rust` to gate its browser `wasm-bindgen` entry point behind
an explicit feature, so the raw dprint plugin artifact does not import browser
glue.

## Performance

Try it yourself with `mise run bench`.

```txt
Benchmarking stdin formatting over 120 CMake files from support/bench-fixtures.
Set BENCH_REPEAT_COUNT, BENCH_FILE_LIMIT, BENCH_RUNS, or BENCH_WARMUP to adjust rounds.
Benchmark 1: native cmakefmt
  Time (mean ± σ):      1.871 s ±  0.003 s    [User: 0.810 s, System: 1.147 s]
  Range (min … max):    1.869 s …  1.875 s    4 runs

Benchmark 2: dprint wasm plugin
  Time (mean ± σ):      1.257 s ±  0.005 s    [User: 0.522 s, System: 0.633 s]
  Range (min … max):    1.250 s …  1.262 s    4 runs

Summary
  dprint wasm plugin ran
    1.49 ± 0.01 times faster than native cmakefmt
```
