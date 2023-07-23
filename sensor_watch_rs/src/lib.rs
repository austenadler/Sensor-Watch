#![cfg_attr(not(target_arch = "wasm32"), no_std)]
#![allow(unused_imports)]

use sensor_watch_sys::watch_display_string;

pub mod face;
pub mod hi_rust;
pub mod kitchen_timer_face;
use core::ffi::{c_char, CStr};
use core::panic::PanicInfo;
use cty::uint8_t;
use modular_bitfield::{
    bitfield,
    specifiers::{B2, B3, B4, B6},
};

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// #[repr(C)]
// pub enum MovementEventType {
//     /// There is no event to report.
//     EventNone = 0,
//     /// Your watch face is entering the foreground.
//     EventActivate,
//     /// Most common event type. Your watch face is being called from the tick callback.
//     EventTick,
//     /// If the watch is in low energy mode and you are in the foreground, you will get a chance to update the display once per minute.
//     EventLowEnergyUpdate,
//     /// Your watch face is being invoked to perform a background task. Don't update the display here; you may not be in the foreground.
//     EventBackgroundTask,
//     /// Your watch face has been inactive for a while. You may want to resign, depending on your watch face's intended use case.
//     EventTimeout,
//     /// The light button has been pressed, but not yet released.
//     EventLightButtonDown,
//     /// The light button was pressed for less than half a second, and released.
//     EventLightButtonUp,
//     /// The light button was held for over half a second, but not yet released.
//     EventLightLongPress,
//     /// The light button was held for over half a second, and released.
//     EventLightLongUp,
//     /// The mode button has been pressed, but not yet released.
//     EventModeButtonDown,
//     /// The mode button was pressed for less than half a second, and released.
//     EventModeButtonUp,
//     /// The mode button was held for over half a second, but not yet released.
//     EventModeLongPress,
//     /// The mode button was held for over half a second, and released. NOTE: your watch face will resign immediately after receiving this event.
//     EventModeLongUp,
//     /// The alarm button has been pressed, but not yet released.
//     EventAlarmButtonDown,
//     /// The alarm button was pressed for less than half a second, and released.
//     EventAlarmButtonUp,
//     /// The alarm button was held for over half a second, but not yet released.
//     EventAlarmLongPress,
//     /// The alarm button was held for over half a second, and released.
//     EventAlarmLongUp,
// }

// #[repr(C)]
// pub struct MovementEvent {
//     pub event_type: MovementEventType,
//     pub subsecond: uint8_t,
// }

// #[bitfield]
// pub struct MovementSettings {
//     #[bits = 1]
//     pub button_should_sound: bool,
//     #[bits = 2]
//     pub to_interval: B2, /*uint8_t*/
//     #[bits = 1]
//     pub to_always: bool,
//     #[bits = 3]
//     pub le_interval: B3, /*uint8_t*/
//     #[bits = 2]
//     pub led_duration: B2, /*uint8_t*/
//     #[bits = 4]
//     pub led_red_color: B4, /*uint8_t*/
//     #[bits = 4]
//     pub led_green_color: B4, /*uint8_t*/
//     #[bits = 6]
//     pub time_zone: B6, /*uint8_t*/
//     #[bits = 1]
//     pub clock_mode_24h: bool,
//     #[bits = 1]
//     pub use_imperial_units: bool,
//     #[bits = 1]
//     pub alarm_enabled: bool,
//     #[bits = 6]
//     pub reserved: B6, /*uint8_t*/
// }

#[no_mangle]
pub extern "C" fn set_display_str() {
    unsafe { watch_display_string([b'R' as i8, b'U' as i8].as_mut_ptr(), 0_u8) }
}

#[panic_handler]
#[cfg(not(target_arch = "wasm32"))]
fn no_panic(_: &PanicInfo) -> ! {
    loop {}
}
