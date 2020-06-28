use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let lib_path = PathBuf::from("lib");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let lib_name = "ftd2xx.lib";
    println!(
        "cargo:rustc-link-lib={}",
        Path::new(lib_name).file_stem().unwrap().to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );

    fs::copy(
        lib_path.join(lib_name).to_str().unwrap(),
        out_path.join(lib_name).to_str().unwrap(),
    )
    .expect("Failed to copy native lib to output directory");

    println!("cargo:rerun-if-changed=lib/ftd2xx.h");

    let bindings = bindgen::Builder::default()
        .header("lib/ftd2xx.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_default(true)
        .derive_debug(true)
        .whitelist_function("FT_.*")
        .whitelist_type("FT_.*")
        .whitelist_var("FT_.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
