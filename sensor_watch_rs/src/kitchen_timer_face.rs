use cty::{c_void, uint32_t, uint8_t};

use cstr::cstr;
use sensor_watch_sys::movement_default_loop_handler;
use sensor_watch_sys::movement_event_t;
use sensor_watch_sys::movement_event_type_t;
use sensor_watch_sys::movement_settings_t;
use sensor_watch_sys::watch_display_string;
use sensor_watch_sys::MovementEvent;

use sensor_watch_sys::error;
use sensor_watch_sys::info;

#[no_mangle]
pub extern "C" fn kitchen_timer_face_setup(
    settings: *mut movement_settings_t,
    watch_face_index: uint8_t,
    context_ptr: *mut &mut c_void,
) {
    error!("Called: kitchen_timer_face_setup");
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_activate(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) {
    error!("Called: kitchen_timer_face_activate");
    unsafe {
        watch_display_string(
            // cstr!("HI").as_ptr().cast_mut(),
            cstr!("HI  DANI").as_ptr().cast_mut(),
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
    let event = MovementEvent::from(event);
    info!("Event: {event:?}");

    match event.event_type {
        _ => unsafe {
            movement_default_loop_handler(event.into(), settings);
        },
    }
    false
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_resign(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) {
    error!("Called: kitchen_timer_face_resign");
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_wants_background_task(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) -> bool {
    error!("Called: kitchen_timer_face_wants_background_task");
    false
}
