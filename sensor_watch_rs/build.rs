use std::process::Command;

fn main() {
    Command::new("cbindgen")
        .args(["--output", "sensor_watch_rs.h"])
        .output()
        .unwrap();
}
