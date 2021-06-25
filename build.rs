extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("Unable to read manifest directory.");
    let profile = env::var("PROFILE").expect("Unable to target profile.");

    let header_path = PathBuf::from(&crate_dir)
        .join("target")
        .join(profile)
        .join("bloom.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_pragma_once(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
}
