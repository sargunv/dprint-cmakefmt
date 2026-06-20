use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn behavior_fixture_inventory_is_complete() {
    let cases = [
        ("basic_cmakelists", "CMakeLists.txt"),
        ("module_file", "FindWidget.cmake"),
        ("no_change", "CMakeLists.txt"),
        ("line_width", "CMakeLists.txt"),
        ("indentation", "CMakeLists.txt"),
        ("no_filesystem_discovery", "CMakeLists.txt"),
    ];

    for (case, file_name) in cases {
        let case_dir = fixture_path(&["behavior", case]);
        assert!(
            case_dir.is_dir(),
            "missing behavior fixture directory: {}",
            case_dir.display(),
        );

        for variant in ["input", "expected"] {
            let fixture_file = case_dir.join(variant).join(file_name);
            assert!(
                fixture_file.is_file(),
                "missing {variant} fixture for {case}: {}",
                fixture_file.display(),
            );
        }
    }

    assert!(
        fixture_path(&["behavior", "unknown_config", "invalid-config.jsonc"]).is_file(),
        "missing unknown config diagnostic fixture",
    );
}

#[test]
fn local_dprint_smoke_config_points_at_release_wasm() {
    let config_path = fixture_path(&["dprint", "local-plugin.jsonc"]);
    let config_text = fs::read_to_string(&config_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", config_path.display()));

    assert!(
        config_text.contains("../../target/wasm32-unknown-unknown/release/dprint_cmakefmt.wasm"),
        "local dprint smoke config must point at the release wasm artifact",
    );
    assert!(
        config_text.contains("\"cmakefmt\""),
        "local dprint smoke config must include plugin-specific cmakefmt config",
    );
    assert!(
        config_text.contains("\"includes\""),
        "local dprint smoke config should constrain smoke-test paths",
    );
}

#[test]
fn no_filesystem_discovery_fixture_contains_conflicting_cmakefmt_config() {
    let source_path = fixture_path(&[
        "behavior",
        "no_filesystem_discovery",
        "input",
        "CMakeLists.txt",
    ]);
    let source = fs::read_to_string(&source_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", source_path.display()));

    let config_path = fixture_path(&[
        "behavior",
        "no_filesystem_discovery",
        "input",
        ".cmake-format.py",
    ]);
    let config = fs::read_to_string(&config_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", config_path.display()));

    assert!(
        source.contains("discovery_probe"),
        "fixture should make the discovery-probe source easy to identify",
    );
    assert!(
        config.contains("line_width = 12"),
        "fixture should contain a conflicting cmake-format config that plugin tests must ignore",
    );
}

#[test]
fn local_dprint_wasm_smoke_formats_and_then_checks_clean() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let temp_dir = unique_temp_dir();

    copy_dir(
        &fixture_path(&["behavior", "basic_cmakelists", "input"]),
        &temp_dir.join("basic"),
    );
    copy_dir(
        &fixture_path(&["behavior", "module_file", "input"]),
        &temp_dir.join("module"),
    );
    let build_status = Command::new("cargo")
        .args(["build", "--release", "--target", "wasm32-unknown-unknown"])
        .current_dir(manifest_dir)
        .status()
        .expect("run cargo build for wasm smoke test");
    assert!(build_status.success(), "wasm release build failed");

    let plugin_path =
        manifest_dir.join("target/wasm32-unknown-unknown/release/dprint_cmakefmt.wasm");
    fs::write(
        temp_dir.join("dprint.jsonc"),
        format!(
            r#"{{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "lineWidth": 80,
  "indentWidth": 2,
  "useTabs": false,
  "includes": ["**/CMakeLists.txt", "**/*.cmake"],
  "cmakefmt": {{}},
  "plugins": ["{}"]
}}
"#,
            plugin_path.display()
        ),
    )
    .expect("write temp-local dprint config");

    let fmt_status = Command::new("dprint")
        .arg("fmt")
        .current_dir(&temp_dir)
        .status()
        .expect("run dprint fmt for wasm smoke test");
    assert!(fmt_status.success(), "dprint fmt smoke test failed");

    let check_status = Command::new("dprint")
        .arg("check")
        .current_dir(&temp_dir)
        .status()
        .expect("run dprint check after smoke formatting");
    assert!(
        check_status.success(),
        "dprint check was not clean after formatting"
    );
}

fn fixture_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(FIXTURES);
    path.extend(parts);
    path
}

fn unique_temp_dir() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "dprint-cmakefmt-smoke-{}-{}",
        std::process::id(),
        unique_suffix(),
    ));
    fs::create_dir_all(&path).expect("create smoke temp dir");
    path
}

fn unique_suffix() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos()
        .to_string()
}

fn copy_dir(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create destination fixture dir");

    for entry in fs::read_dir(source).expect("read source fixture dir") {
        let entry = entry.expect("read source fixture entry");
        let destination_path = destination.join(entry.file_name());
        if entry
            .file_type()
            .expect("read source fixture file type")
            .is_dir()
        {
            copy_dir(&entry.path(), &destination_path);
        } else {
            fs::copy(entry.path(), destination_path).expect("copy source fixture file");
        }
    }
}
