#![allow(clippy::if_same_then_else)]

extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::EnumVariation;

fn main() {
    let link_lib = if cfg!(all(feature = "windows", target_os = "windows")) {
        unimplemented!()
    } else if cfg!(all(feature = "macos", target_os = "macos")) {
        unimplemented!()
    } else if cfg!(all(feature = "x11", target_os = "linux")) {
        "xcb"
    } else {
        unreachable!("Unsupported platform")
    };

    let clang_definition = if cfg!(all(feature = "windows", target_os = "windows")) {
        "USE_WINDOWS"
    } else if cfg!(all(feature = "macos", target_os = "macos")) {
        "USE_MACOS"
    } else if cfg!(all(feature = "x11", target_os = "linux")) {
        "USE_XCB"
    } else {
        unreachable!("Unsupported platform")
    };

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib={}", link_lib);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Add the platform definition
        .clang_arg(format!("-D{}", clang_definition))
        // Use Rust's core instead of libstd
        .use_core()
        // Make the default type of enums use Rust style
        .default_enum_style(EnumVariation::Rust {non_exhaustive: true})
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
