extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Read configuration information from ruby executable.
fn ruby_configuration(key: &str) -> String {
    let config = Command::new("ruby")
        .arg("-e")
        .arg(format!("print RbConfig::CONFIG['{}']", key))
        .output()
        .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    String::from_utf8(config.stdout).expect("Configuration value is not valid UTF-8.")
}

fn main() {
    let ruby_h_path = ruby_configuration("rubyhdrdir");
    let platform_h_path = ruby_configuration("rubyarchhdrdir");
    let ruby_lib_path = ruby_configuration("libdir");
    let ruby_library = ruby_configuration("RUBY_SO_NAME");

    // Tell cargo to tell rustc to link the ruby library.
    println!("cargo:rustc-link-search={}", ruby_lib_path);
    println!("cargo:rustc-link-lib=dylib={}", ruby_library);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("-I{}", ruby_h_path))
        .clang_arg(format!("-I{}", platform_h_path))
        .clang_arg(format!("-I{}/ruby/backward", platform_h_path))
        .clang_arg("-fdeclspec")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}