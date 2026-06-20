#!/usr/bin/env bash
set -euo pipefail

cmakefmt_version="1.6.0"
archive_name="cmakefmt-rust-${cmakefmt_version}.crate"
url="https://static.crates.io/crates/cmakefmt-rust/${archive_name}"

cache_dir="third_party/cache"
source_dir="third_party/cmakefmt-rust-${cmakefmt_version}"
archive_path="${cache_dir}/${archive_name}"

mkdir -p "$cache_dir" third_party

if [[ ! -f "$archive_path" ]]; then
  curl --fail --location --retry 3 --output "$archive_path" "$url"
fi

if [[ ! -d "$source_dir" ]]; then
  tar -C third_party -xzf "$archive_path"
fi

test -f "$source_dir/Cargo.toml"
test -f "$source_dir/src/lib.rs"

apply_patch() {
  local patch_file="$1"

  if patch -d "$source_dir" -p1 --forward --batch --dry-run --silent < "$patch_file" >/dev/null 2>&1; then
    patch -d "$source_dir" -p1 --forward --batch --silent < "$patch_file"
    echo "applied $(basename "$patch_file")"
  elif patch -d "$source_dir" -p1 --reverse --batch --dry-run --silent < "$patch_file" >/dev/null 2>&1; then
    echo "already applied: $(basename "$patch_file")"
  else
    echo "failed to apply $(basename "$patch_file")" >&2
    exit 1
  fi
}

for patch_file in support/patches/*.patch; do
  apply_patch "$patch_file"
done

echo "cmakefmt-rust source ready: $source_dir"
