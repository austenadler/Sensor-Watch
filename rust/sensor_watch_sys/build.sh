#!/bin/bash
set -euxo pipefail
cargo +nightly build --release --target thumbv6m-none-eabi -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
cargo +nightly build --release --target wasm32-unknown-emscripten -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
