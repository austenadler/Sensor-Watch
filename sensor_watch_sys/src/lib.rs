#![cfg_attr(not(target_arch = "wasm32"), no_std)]
#![allow(non_camel_case_types)]

use core::ffi::c_uint;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EventType {
    Activate,
    AlarmButtonDown,
    AlarmButtonUp,
    AlarmLongPress,
    AlarmLongUp,
    BackgroundTask,
    LightButtonDown,
    LightButtonUp,
    LightLongPress,
    LightLongUp,
    LowEnergyUpdate,
    ModeButtonDown,
    ModeButtonUp,
    ModeLongPress,
    ModeLongUp,
    None,
    Tick,
    Timeout,
    Other(c_uint),
}

impl From<movement_event_type_t> for EventType {
    fn from(value: movement_event_type_t) -> Self {
        match value {
            movement_event_type_t::EVENT_ACTIVATE => Self::Activate,
            movement_event_type_t::EVENT_ALARM_BUTTON_DOWN => Self::AlarmButtonDown,
            movement_event_type_t::EVENT_ALARM_BUTTON_UP => Self::AlarmButtonUp,
            movement_event_type_t::EVENT_ALARM_LONG_PRESS => Self::AlarmLongPress,
            movement_event_type_t::EVENT_ALARM_LONG_UP => Self::AlarmLongUp,
            movement_event_type_t::EVENT_BACKGROUND_TASK => Self::BackgroundTask,
            movement_event_type_t::EVENT_LIGHT_BUTTON_DOWN => Self::LightButtonDown,
            movement_event_type_t::EVENT_LIGHT_BUTTON_UP => Self::LightButtonUp,
            movement_event_type_t::EVENT_LIGHT_LONG_PRESS => Self::LightLongPress,
            movement_event_type_t::EVENT_LIGHT_LONG_UP => Self::LightLongUp,
            movement_event_type_t::EVENT_LOW_ENERGY_UPDATE => Self::LowEnergyUpdate,
            movement_event_type_t::EVENT_MODE_BUTTON_DOWN => Self::ModeButtonDown,
            movement_event_type_t::EVENT_MODE_BUTTON_UP => Self::ModeButtonUp,
            movement_event_type_t::EVENT_MODE_LONG_PRESS => Self::ModeLongPress,
            movement_event_type_t::EVENT_MODE_LONG_UP => Self::ModeLongUp,
            movement_event_type_t::EVENT_NONE => Self::None,
            movement_event_type_t::EVENT_TICK => Self::Tick,
            movement_event_type_t::EVENT_TIMEOUT => Self::Timeout,
            c => Self::Other(c.0),
        }
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        #[cfg(target_arch="wasm32")]
        println!($($arg)*);
    }};
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        #[cfg(target_arch="wasm32")]
        eprintln!($($arg)*);
    }};
}

// #[no_mangle]
// pub extern "C" fn set_display_str() {
//     unsafe { watch_display_string([b'R' as i8, b'U' as i8].as_mut_ptr(), 0_u8) }
// }

// #[panic_handler]
// #[cfg(not(target_arch = "wasm32"))]
// fn no_panic(_: &PanicInfo) -> ! {
//     loop {}
// }
