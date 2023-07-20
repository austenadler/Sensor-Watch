extern crate cbindgen;

use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;
use cbindgen::Config;

fn main() {
    bindgen();
    cbindgen();
}

fn cbindgen() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("sensor_watch_rs.h");
}

fn bindgen() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search="../watch-library/shared/watch/",");
    // println!("cargo:rustc-link-search=all=./watch");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .use_core()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // .clang_arg("-isystem")
        // .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/arm-linux-gnueabi/usr/include/")
        // .clang_arg("-I/usr/arm-linux-gnueabihf/usr/include/gnu")
        .clang_args([
            "-I../tinyusb/src",
            "-I../boards/OSO-SWAT-A1-05",
            "-I../watch-library/shared/config/",
            "-I../watch-library/shared/driver/",
            "-I../watch-library/shared/watch/",
            "-I../watch-library/hardware/include",
            "-I../watch-library/hardware/hal/",
            "-I../watch-library/hardware/hal/documentation/",
            "-I../watch-library/hardware/hal/include/",
            "-I../watch-library/hardware/hal/src/",
            "-I../watch-library/hardware/hal/utils/",
            "-I../watch-library/hardware/hal/utils/include/",
            "-I../watch-library/hardware/hal/utils/src/",
            "-I../watch-library/hardware/hpl/",
            "-I../watch-library/hardware/hpl/core/",
            "-I../watch-library/hardware/hpl/dmac/",
            "-I../watch-library/hardware/hpl/eic/",
            "-I../watch-library/hardware/hpl/gclk/",
            "-I../watch-library/hardware/hpl/mclk/",
            "-I../watch-library/hardware/hpl/osc32kctrl/",
            "-I../watch-library/hardware/hpl/oscctrl/",
            "-I../watch-library/hardware/hpl/pm/",
            "-I../watch-library/hardware/hpl/port/",
            "-I../watch-library/hardware/hpl/sercom/",
            "-I../watch-library/hardware/hpl/slcd/",
            "-I../watch-library/hardware/hpl/systick/",
            "-I../watch-library/hardware/hri/",
            "-I../watch-library/hardware/hw/",
            "-I../watch-library/hardware/watch/",
            "-I../watch-library/hardware",
        ])
        // .allowlist_type("movement_settings_t")
        // .allowlist_type("movement_event_t")
        // .allowlist_type("movement_event_type_t")
        .allowlist_file("./../movement/movement.h")
        .allowlist_file("./../watch-library/shared/watch/watch_slcd.h")
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        //
        // .newtype_enum("movement_event_type_t")
        // .constified_enum("movement_event_type_t")
        // .constified_enum_module("movement_event_type_t")
        .generate()
        .expect("Unable to generate bindings");
    // panic!();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
