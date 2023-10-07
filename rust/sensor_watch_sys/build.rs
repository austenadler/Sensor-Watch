use std::{env, path::PathBuf, process::Command};

use bindgen::EnumVariation;

fn main() {
    bindgen();
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

    let includes = match env::var("TARGET").unwrap().as_str() {
        "thumbv6m-none-eabi" => [
            "/usr/arm-linux-gnueabi/usr/include/",
            "../../tinyusb/src",
            "../../boards/OSO-SWAT-A1-05",
            "../../watch-library/shared/config/",
            "../../watch-library/shared/driver/",
            "../../watch-library/shared/watch/",
            "../../watch-library/hardware/include",
            "../../watch-library/hardware/hal/",
            "../../watch-library/hardware/hal/documentation/",
            "../../watch-library/hardware/hal/include/",
            "../../watch-library/hardware/hal/src/",
            "../../watch-library/hardware/hal/utils/",
            "../../watch-library/hardware/hal/utils/include/",
            "../../watch-library/hardware/hal/utils/src/",
            "../../watch-library/hardware/hpl/",
            "../../watch-library/hardware/hpl/core/",
            "../../watch-library/hardware/hpl/dmac/",
            "../../watch-library/hardware/hpl/eic/",
            "../../watch-library/hardware/hpl/gclk/",
            "../../watch-library/hardware/hpl/mclk/",
            "../../watch-library/hardware/hpl/osc32kctrl/",
            "../../watch-library/hardware/hpl/oscctrl/",
            "../../watch-library/hardware/hpl/pm/",
            "../../watch-library/hardware/hpl/port/",
            "../../watch-library/hardware/hpl/sercom/",
            "../../watch-library/hardware/hpl/slcd/",
            "../../watch-library/hardware/hpl/systick/",
            "../../watch-library/hardware/hri/",
            "../../watch-library/hardware/hw/",
            "../../watch-library/hardware/watch/",
            "../../watch-library/hardware",
        ]
        .iter()
        .map(|s| format!("-I{s}"))
        .collect(),
        "i686-unknown-linux-gnu" | "wasm32-unknown-emscripten" => {
            let mut includes = shell_words::split(
                &String::from_utf8(
                    Command::new("emcc")
                        .arg("--cflags")
                        .output()
                        .unwrap()
                        .stdout,
                )
                .unwrap(),
            )
            .unwrap();
            includes.extend(
                [
                    "../../boards/OSO-SWAT-A1-05",
                    "../../watch-library/shared/driver/",
                    "../../watch-library/shared/config/",
                    "../../watch-library/shared/watch/",
                    "../../watch-library/simulator/watch/",
                    "../../watch-library/simulator/hpl/port/",
                    "../../watch-library/hardware/include/component",
                    "../../watch-library/hardware/hal/include/",
                    "../../watch-library/hardware/hal/utils/include/",
                    "../../watch-library/hardware/hpl/slcd/",
                    "../../watch-library/hardware/hw/",
                ]
                .iter()
                .map(|s| format!("-I{s}")),
            );
            includes
        }
        t => panic!("Target {t} is not supported"),
    };

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .use_core()
        .header("wrapper.h")
        .clang_args(includes)
        .allowlist_file("./../../movement/movement.h")
        .allowlist_file("./../../watch-library/shared/watch/watch.h")
        .allowlist_file("./../../watch-library/shared/watch/watch_slcd.h")
        .allowlist_file("./../../watch-library/shared/watch/watch_utility.h")
        .allowlist_file("./../../watch-library/shared/watch/watch_rtc.h")
        .allowlist_file("./../../watch-library/shared/watch/watch_buzzer.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .merge_extern_blocks(true)
        .generate()
        .expect("Unable to generate bindings");
    // panic!();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
