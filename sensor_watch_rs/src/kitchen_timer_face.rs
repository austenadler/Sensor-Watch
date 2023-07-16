use cty::{c_void, uint8_t};

use super::err;

pub struct MovementSettings {}
pub struct MovementEvent {}

#[no_mangle]
pub extern "C" fn kitchen_timer_face_setup(
    settings: *mut MovementSettings,
    watch_face_index: uint8_t,
    context_ptr: *mut &mut c_void,
) {
    err!("Called: kitchen_timer_face_setup");
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_activate(
    settings: *mut MovementSettings,
    context: *mut &mut c_void,
) {
    err!("Called: kitchen_timer_face_activate");
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_loop(
    event: *mut MovementEvent,
    settings: *mut MovementSettings,
    context: *mut &mut c_void,
) -> bool {
    err!("Called: kitchen_timer_face_loop");
    false
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_resign(
    settings: *mut MovementSettings,
    context: *mut &mut c_void,
) {
    err!("Called: kitchen_timer_face_resign");
}
