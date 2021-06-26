extern crate cbindgen;

use std::env;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn configuration() -> cbindgen::Config {
    let mut config: cbindgen::Config = Default::default();
    config.function.prefix = Some("__declspec(dllexport)".to_string());
    config
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn configuration() -> cbindgen::Config {
    Default::default()
}

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("Unable to read manifest directory.");
    let profile = env::var("PROFILE").expect("Unable to target profile.");

    let header_path = PathBuf::from(&crate_dir)
        .join("target")
        .join(profile)
        .join("bloom.h");

    let mut config: cbindgen::Config = configuration();
    config.language = cbindgen::Language::C;
    config.pragma_once = true;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
}
