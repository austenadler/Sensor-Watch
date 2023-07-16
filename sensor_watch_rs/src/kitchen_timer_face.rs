use cty::{c_void, uint8_t};

pub struct MovementSettings {}

#[no_mangle]
pub extern "C" fn kitchen_timer_face_setup(
    settings: *mut MovementSettings,
    watch_face_index: uint8_t,
    context_ptr: *mut &mut c_void,
) {
    no_stdout::dprint!("Called setup function");
}
// pub extern "C" fn void kitchen_timer_face_activate(settings: const *MovementSettings, void *context) {

// }
// pub extern "C" fn bool kitchen_timer_face_loop(movement_event_t event, settings: const *MovementSettings, void *context) {

// }
// pub extern "C" fn void kitchen_timer_face_resign(settings: const *MovementSettings, void *context) {

// }
