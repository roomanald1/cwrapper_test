extern crate bindgen;
use std::{env, fs};
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=igraph");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/local/include/igraph")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let binding_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&binding_path)
        .expect("Couldn't write bindings!");


    // Add allow attributes to the generated file
    let mut contents = fs::read_to_string(&binding_path).unwrap();
    contents = format!("#[allow(unsafe_code, unused, non_snake_case)]\n{}", contents);

    fs::write(&binding_path, contents).unwrap();
}