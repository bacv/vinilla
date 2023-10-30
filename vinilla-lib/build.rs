use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_dir = PathBuf::from("../".to_string());

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file(root_dir.join("libvinilla.h"));
}
