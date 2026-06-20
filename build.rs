fn main() {
    println!("cargo:rerun-if-env-changed=GITHUB_REF_NAME");
}
