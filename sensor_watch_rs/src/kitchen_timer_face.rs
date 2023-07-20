use cty::{c_void, uint32_t, uint8_t};

use crate::movement_event_t;
use crate::movement_settings_t;

use super::err;

// #[repr(C)]
// pub struct MovementSettingsInner {
//     button_should_sound: bool, //  : 1;
//     to_interval: uint8_t,      //  : 2;
//     to_always: bool,           //  : 1;
//     le_interval: uint8_t,      //  : 3;
//     led_duration: uint8_t,     //  : 2;
//     led_red_color: uint8_t,    //  : 4;
//     led_green_color: uint8_t,  //  : 4;
//     time_zone: uint8_t,        //  : 6;
//     clock_mode_24h: bool,      //  : 1;
//     use_imperial_units: bool,  //  : 1;
//     alarm_enabled: bool,       //  : 1;
//     reserved: uint8_t,         //  : 6;
// }
// #[repr(C)]
// pub struct movement_settings_t {
//     reg: uint32_t,
//     bit: MovementSettingsInner,
// }

// pub struct movement_event_t {}

#[no_mangle]
pub extern "C" fn kitchen_timer_face_setup(
    settings: *mut movement_settings_t,
    watch_face_index: uint8_t,
    context_ptr: *mut &mut c_void,
) {
    err!("Called: kitchen_timer_face_setup");
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_activate(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) {
    err!("Called: kitchen_timer_face_activate");
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_loop(
    event: movement_event_t,
    settings: *mut movement_settings_t,
    context: *mut c_void,
) -> bool {
    err!("Called: kitchen_timer_face_loop");
    false
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_resign(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) {
    err!("Called: kitchen_timer_face_resign");
}
