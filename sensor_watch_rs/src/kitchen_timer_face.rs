use cty::{c_void, uint32_t, uint8_t};

use crate::movement_default_loop_handler;
use crate::movement_event_t;
use crate::movement_event_type_t;
use crate::movement_settings_t;
use crate::watch_display_string;
use cstr::cstr;

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
    unsafe {
        watch_display_string(
            // cstr!("HI").as_ptr().cast_mut(),
            cstr!("HI  RUST").as_ptr().cast_mut(),
            // cstr!("HI  LESLIE").as_ptr().cast_mut(),
            // [b'R' as i8, b'U' as i8, b'S' as i8, b'T' as i8, 0].as_mut_ptr(),
            0,
        )
    };
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_loop(
    event: movement_event_t,
    settings: *mut movement_settings_t,
    context: *mut c_void,
) -> bool {
    err!("Called: kitchen_timer_face_loop");
    match movement_event_type_t(event.event_type as u32) {
        movement_event_type_t::EVENT_ALARM_LONG_UP => {
            err!("Got button event alarm long up");
        }
        _ => unsafe {
            movement_default_loop_handler(event, settings);
        },
    }
    false
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_resign(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) {
    err!("Called: kitchen_timer_face_resign");
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_wants_background_task(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) -> bool {
    err!("Called: kitchen_timer_face_wants_background_task");
    false
}
