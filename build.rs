use std::{path::PathBuf, fs::OpenOptions, fs::File, io::Write};
use bindgen::{Bindings, Builder};

fn main() {
    add_libwacom_support()
}

fn add_libwacom_support() {
    let header_path: PathBuf = PathBuf::from("c").join("wacom.h");
    let include_path: PathBuf = PathBuf::from("/").join("usr").join("include").join("libwacom-1.0").join("libwacom");
    let output_path: PathBuf = PathBuf::from("src").join("info").join("wacom_bindings.rs");

    let file_header: &str = "#![allow(non_upper_case_globals)]\n#![allow(non_camel_case_types)]\n#![allow(non_snake_case)]\n#![allow(dead_code)]\n\n";

    // Include libraries
    println!("cargo:rustc-link-lib=wacom");

    // Rebuild crate when header updated
    println!("cargo:rerun-if-changed={}", header_path.to_str().unwrap());

    let bindings: Bindings = Builder::default()
        .header(header_path.to_str().unwrap())
        .clang_arg(format!("-I{}", include_path.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate().expect("Could not generate bindings...");

    let mut file: File = OpenOptions::new().write(true).truncate(true).create(true).open(output_path).expect("Could not write bindings...");
    file.write_all(file_header.as_bytes()).expect("Could not write bindings...");
    bindings.write(Box::new(file)).expect("Could not write bindings...");
}