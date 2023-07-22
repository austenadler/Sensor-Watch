extern crate cbindgen;

use cbindgen::Config;
use std::env;

fn main() {
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
