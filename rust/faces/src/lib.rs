#![cfg_attr(not(target_arch = "wasm32"), no_std)]
#![allow(unused_imports)]

// pub mod hi_rust;
pub mod kitchen_timer_face;
use core::panic::PanicInfo;
use sensor_watch_rs;

// #[panic_handler]
// #[cfg(not(target_arch = "wasm32"))]
// fn no_panic(_: &PanicInfo) -> ! {
//     loop {}
// }
