#![cfg_attr(not(target_arch = "wasm32"), no_std)]
#![allow(non_camel_case_types)]
use core::ffi::{c_uint, c_void, CStr};
pub mod display;
pub mod time;

pub const WATCH_NUM_DIGITS: u8 = 10;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" {
    pub fn malloc(size: usize) -> *mut c_void;
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct MovementEvent {
    pub event_type: EventType,
    pub subsecond: u8,
}

impl From<movement_settings_t__bindgen_ty_1> for movement_settings_t {
    fn from(value: movement_settings_t__bindgen_ty_1) -> Self {
        Self { bit: value }
    }
}

impl From<MovementEvent> for movement_event_t {
    fn from(value: MovementEvent) -> Self {
        Self {
            event_type: value.event_type.into(),
            subsecond: value.subsecond,
        }
    }
}

impl From<movement_event_t> for MovementEvent {
    fn from(value: movement_event_t) -> Self {
        Self {
            event_type: value.event_type.into(),
            subsecond: value.subsecond,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// TODO: Repr?
// #[repr(u8)]
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

impl From<u8> for EventType {
    fn from(value: u8) -> Self {
        movement_event_type_t(value as u32).into()
    }
}
impl From<EventType> for u8 {
    fn from(value: EventType) -> Self {
        movement_event_type_t::from(value).0 as u8
    }
}

impl From<EventType> for movement_event_type_t {
    fn from(value: EventType) -> Self {
        match value {
            EventType::Activate => movement_event_type_t::EVENT_ACTIVATE,
            EventType::AlarmButtonDown => movement_event_type_t::EVENT_ALARM_BUTTON_DOWN,
            EventType::AlarmButtonUp => movement_event_type_t::EVENT_ALARM_BUTTON_UP,
            EventType::AlarmLongPress => movement_event_type_t::EVENT_ALARM_LONG_PRESS,
            EventType::AlarmLongUp => movement_event_type_t::EVENT_ALARM_LONG_UP,
            EventType::BackgroundTask => movement_event_type_t::EVENT_BACKGROUND_TASK,
            EventType::LightButtonDown => movement_event_type_t::EVENT_LIGHT_BUTTON_DOWN,
            EventType::LightButtonUp => movement_event_type_t::EVENT_LIGHT_BUTTON_UP,
            EventType::LightLongPress => movement_event_type_t::EVENT_LIGHT_LONG_PRESS,
            EventType::LightLongUp => movement_event_type_t::EVENT_LIGHT_LONG_UP,
            EventType::LowEnergyUpdate => movement_event_type_t::EVENT_LOW_ENERGY_UPDATE,
            EventType::ModeButtonDown => movement_event_type_t::EVENT_MODE_BUTTON_DOWN,
            EventType::ModeButtonUp => movement_event_type_t::EVENT_MODE_BUTTON_UP,
            EventType::ModeLongPress => movement_event_type_t::EVENT_MODE_LONG_PRESS,
            EventType::ModeLongUp => movement_event_type_t::EVENT_MODE_LONG_UP,
            EventType::None => movement_event_type_t::EVENT_NONE,
            EventType::Tick => movement_event_type_t::EVENT_TICK,
            EventType::Timeout => movement_event_type_t::EVENT_TIMEOUT,
            EventType::Other(c) => movement_event_type_t(c),
        }
    }
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

/// Convert a u8 into a 2-digit number
/// If it doesn't fit, just ignore the leftmost digit
/// Unsafe because idx is not checked
pub fn watch_display_u8(input: u8, two_digits: bool, idx: u8) {
    {
        // Check to make sure this will fit on the screen so we can make this function safe
        let num_digits = if two_digits { 2 } else { 1 };
        // Example: If we write 1 digit to digit idx 9, 1+9=10 => okay
        // If we write two digits to digit idx 9, 2+9=11 => not okay
        if idx + num_digits > 10 {
            error!(
                "Not attempting to write {num_digits} at idx {idx} as there is no digit to display"
            );
            return;
        }
    }
    let mut buf = [0x0; 2 + 1];

    write_u8_chars(&mut buf[0..=1], input, two_digits);
    // Just in case the write_u8_chars api changes, ensure the last element is zero
    buf[2] = 0x0;

    // buf is already zeroed, so we don't have to worry about null termination
    let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(&buf) };

    info!(
        "Displaying {:?} at position {idx} (two_digits: {two_digits})",
        cstr
    );

    // Now, actually write it
    unsafe {
        watch_display_string(cstr.as_ptr().cast_mut(), idx);
    }
}

/// Write either 1 or 2 digits to the buf
pub fn write_u8_chars(buf: &mut [u8], input: u8, two_digits: bool) {
    let right_digit = b'0' + (input % 10);
    if two_digits {
        // Two digits
        let left_digit = b'0' + ((input / 10) % 10);
        buf[0] = left_digit;
        buf[1] = right_digit;
    } else {
        // One digit
        buf[0] = right_digit;
    }
}

/// Print to `console.log` if using emulator
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        #[cfg(not(target_arch="arm"))]
        println!($($arg)*);
    }};
}

/// Print to `console.err` if using emulator
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        #[cfg(not(target_arch="arm"))]
        eprintln!($($arg)*);
    }};
}
