# dprint-cmakefmt

`dprint-cmakefmt` is a Rust dprint plugin for formatting CMake files with
[`cmakefmt`](https://github.com/cmakefmt/cmakefmt).

The plugin is intended to build as a raw `wasm32-unknown-unknown` dprint plugin
module. Runtime formatting stays in memory: dprint supplies file text, path
metadata, and configuration; the plugin calls the cmakefmt library API.
