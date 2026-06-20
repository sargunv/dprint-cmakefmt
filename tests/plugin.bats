setup_file() {
  bats_require_minimum_version 1.5.0
}

setup() {
  repo_root="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  cd "$repo_root"
  plugin_path="$repo_root/target/wasm32-unknown-unknown/release/dprint_cmakefmt.wasm"
}

write_config() {
  local config_path="$1"
  local body="$2"

  cat > "$config_path" <<JSON
{
  "plugins": ["$plugin_path"],
  $body
}
JSON
}

format_stdin_with_dprint() {
  local config_path="$1"
  local source_path="$2"

  dprint fmt --stdin "$source_path" --config "$config_path" --config-discovery=false < "$source_path"
}

@test "plugin wasm only imports the expected dprint host functions" {
  run wasm-tools print "$plugin_path"

  [ "$status" -eq 0 ]
  [[ "$output" == *'(import "dprint" "host_has_cancelled"'* ]]
  [[ "$output" != *"__wbindgen"* ]]
}

@test "dprint CLI resolves globals into cmakefmt config" {
  config_path="$BATS_TEST_TMPDIR/dprint.json"
  write_config "$config_path" '
  "lineWidth": 20,
  "indentWidth": 3,
  "useTabs": true,
  "cmakefmt": {}'

  run dprint output-resolved-config --config "$config_path" --config-discovery=false

  [ "$status" -eq 0 ]
  [[ "$output" == *'"lineWidth": 20'* ]]
  [[ "$output" == *'"indentWidth": 3'* ]]
  [[ "$output" == *'"useTabs": true'* ]]
}

@test "dprint CLI reports CMake file extensions and names" {
  config_path="$BATS_TEST_TMPDIR/dprint.json"
  project_dir="$BATS_TEST_TMPDIR/project"
  mkdir -p "$project_dir/Modules" "$project_dir/cmake"
  touch \
    "$project_dir/CMakeLists.txt" \
    "$project_dir/CMakeLists.txt.in" \
    "$project_dir/Modules/FindThing.cmake" \
    "$project_dir/cmake/not-cmake.txt"
  write_config "$config_path" '
  "cmakefmt": {}'

  run dprint output-file-paths --config "$config_path" --config-discovery=false "$project_dir"

  [ "$status" -eq 0 ]
  [[ "$output" == *"CMakeLists.txt"* ]]
  [[ "$output" == *"CMakeLists.txt.in"* ]]
  [[ "$output" == *"Modules/FindThing.cmake"* ]]
  [[ "$output" != *"cmake/not-cmake.txt"* ]]
}

@test "dprint CLI lets cmakefmt config override globals" {
  config_path="$BATS_TEST_TMPDIR/dprint.json"
  write_config "$config_path" '
  "lineWidth": 20,
  "indentWidth": 3,
  "useTabs": false,
  "cmakefmt": {
    "lineWidth": 100,
    "indentWidth": 4,
    "useTabs": true
  }'

  run dprint output-resolved-config --config "$config_path" --config-discovery=false

  [ "$status" -eq 0 ]
  [[ "$output" == *'"lineWidth": 100'* ]]
  [[ "$output" == *'"indentWidth": 4'* ]]
  [[ "$output" == *'"useTabs": true'* ]]
}

@test "dprint CLI rejects unsupported cmakefmt options" {
  config_path="$BATS_TEST_TMPDIR/dprint.json"
  write_config "$config_path" '
  "cmakefmt": { "definitelyUnknownOption": true }'

  run dprint output-resolved-config --config "$config_path" --config-discovery=false

  [ "$status" -eq 1 ]
  [[ "$output" == *"Unknown property in configuration (definitelyUnknownOption)"* ]]
}

@test "dprint CLI formats stdin through the plugin" {
  config_path="$BATS_TEST_TMPDIR/dprint.json"
  source_path="$BATS_TEST_TMPDIR/CMakeLists.txt"
  write_config "$config_path" '
  "cmakefmt": {}'
  printf 'CMAKE_MINIMUM_REQUIRED(VERSION 3.24)\n' > "$source_path"

  run --separate-stderr format_stdin_with_dprint "$config_path" "$source_path"

  if [ "$status" -ne 0 ]; then
    printf "%s\n" "$stderr"
  fi
  [ "$status" -eq 0 ]
  [[ "$output" == 'cmake_minimum_required(VERSION 3.24)' ]]
}

@test "dprint CLI does not use cmakefmt filesystem config discovery" {
  config_path="$BATS_TEST_TMPDIR/dprint.json"
  source_dir="$BATS_TEST_TMPDIR/project"
  source_path="$source_dir/CMakeLists.txt"
  mkdir -p "$source_dir"
  cat > "$source_dir/.cmakefmt.yaml" <<YAML
format:
  line_width: 12
  tab_size: 8
YAML
  printf 'target_link_libraries(example PRIVATE alpha beta gamma delta)\n' > "$source_path"
  write_config "$config_path" '
  "lineWidth": 120,
  "indentWidth": 2,
  "cmakefmt": {}'

  run dprint output-resolved-config --config "$config_path" --config-discovery=false

  [ "$status" -eq 0 ]
  [[ "$output" == *'"lineWidth": 120'* ]]
  [[ "$output" == *'"indentWidth": 2'* ]]

  run --separate-stderr format_stdin_with_dprint "$config_path" "$source_path"

  if [ "$status" -ne 0 ]; then
    printf "%s\n" "$stderr"
  fi
  [ "$status" -eq 0 ]
}
