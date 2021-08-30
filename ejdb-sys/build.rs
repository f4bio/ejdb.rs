extern crate bindgen;
extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    pkg_config::Config::new().probe("zlib").unwrap();

    // let dst = Config::new("ejdb-upstream")
    //     .cflag("-w")
    //     .profile("Release")
    //     .define("BUILD_SAMPLES", "OFF")
    //     .define("BUILD_SHARED_LIBS", "OFF")
    //     .define("CMAKE_BUILD_TYPE", "Release")
    //     .define("PACKAGE_DEB", "ON")
    //     .build();
    //
    // Command::new("make").status().expect("failed to make!");
    println!("cargo:rustc-link-search=native=ejdb2-release/lib");
    // println!("cargo:rustc-link-lib=static=ejdb2");

    let bindings = bindgen::Builder::default()
        .header("ejdb2-release/include/ejdb2/ejdb2.h")
        // Hide duplicated types
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./out/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
