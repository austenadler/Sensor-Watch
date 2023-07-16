#![cfg_attr(not(target_arch = "wasm32"), no_std)]
#![allow(unused_imports)]
use core::ffi::{c_char, CStr};
use core::panic::PanicInfo;
use cty::uint8_t;

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
