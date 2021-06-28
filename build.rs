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

    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    config.pragma_once = true;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
}
