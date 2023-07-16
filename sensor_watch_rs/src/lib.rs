#![cfg_attr(not(target_arch = "wasm32"), no_std)]
#![allow(unused_imports)]

pub mod kitchen_timer_face;
use core::ffi::{c_char, CStr};
use core::panic::PanicInfo;
use cty::uint8_t;

/// Helpers function for wasm_bindgen
macro_rules! err {
    ($msg:tt) => {
        #[cfg(target_arch = "wasm32")]
        {
            eprintln!("{}", $msg);
        }
    };
}
pub(crate) use err;
macro_rules! log {
    ($msg:tt) => {
        #[cfg(target_arch = "wasm32")]
        {
            println!("{}", $msg);
        }
    };
}
pub(crate) use log;

#[no_mangle]
pub extern "C" fn set_display_str() {
    unsafe { watch_display_string([b'R' as i8, b'U' as i8].as_ptr(), 0_u8) }
}

extern "C" {
    fn watch_display_string(string: *const c_char, position: uint8_t);
}

#[panic_handler]
#[cfg(not(target_arch = "wasm32"))]
fn no_panic(_: &PanicInfo) -> ! {
    loop {}
}
