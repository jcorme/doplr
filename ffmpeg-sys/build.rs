extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rerun-if-changed=includes.h");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src");

    bindgen::Builder::default()
        .header("includes.h")
        .generate_comments(true)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
