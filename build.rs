use std::env;
use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub const LIB_FILE_NAME: &str = "ftd2xx.lib";

#[cfg(not(target_os = "windows"))]
pub const LIB_FILE_NAME: &str = "libftd2xx.a";

#[cfg(target_os = "windows")]
pub const PREBUILT_BINDINGS_FILE_NAME: &str = "bindings_win.rs";

#[cfg(not(target_os = "windows"))]
pub const PREBUILT_BINDINGS_FILE_NAME: &str = "bindings_lnx.rs";

pub const BINDINGS_FILE_NAME: &str = "bindings.rs";

// If binding generation is enabled, run bindgen to generate fresh bindings
#[cfg(feature = "gen-bindings")]
fn process_bindings(_lib_path: &PathBuf, out_path: &PathBuf) {
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
        .write_to_file(out_path.join(BINDINGS_FILE_NAME))
        .expect("Couldn't write bindings!");
}

// If binding generation is not enabled, copy the prebuilt bindings from the lib folder
#[cfg(not(feature = "gen-bindings"))]
fn process_bindings(lib_path: &PathBuf, out_path: &PathBuf) {
    fs::copy(
        lib_path.join(PREBUILT_BINDINGS_FILE_NAME).to_str().unwrap(),
        out_path.join(BINDINGS_FILE_NAME).to_str().unwrap(),
    )
    .expect("Failed to copy prebuilt bindings to output directory");
}

fn main() {
    let lib_path = PathBuf::from("lib");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy the static library into the output folder and instruct cargo to link against it
    fs::copy(
        lib_path.join(LIB_FILE_NAME).to_str().unwrap(),
        out_path.join(LIB_FILE_NAME).to_str().unwrap(),
    )
    .expect("Failed to copy native lib to output directory");

    println!("cargo:rustc-link-lib=static=ftd2xx");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );

    process_bindings(&lib_path, &out_path);
}
