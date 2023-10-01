use crate::{info, sys::movement_settings_t__bindgen_ty_1, MovementEvent};
use cty::uint8_t;

pub trait WatchFace {
    fn face_initial_setup(
        settings: movement_settings_t__bindgen_ty_1,
        watch_face_index: uint8_t,
        // context_ptr: *mut *mut c_void,
    ) -> Self;
    fn face_setup(
        &mut self,
        _settings: movement_settings_t__bindgen_ty_1,
        _watch_face_index: uint8_t,
        // context_ptr: *mut *mut c_void,
        // settings: *mut movement_settings_t,
        // watch_face_index: uint8_t,
        // context_ptr: *mut *mut c_void,
    ) {
    }
    fn face_activate(
        &mut self,
        settings: movement_settings_t__bindgen_ty_1,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    );
    fn face_loop(
        &mut self,
        event: MovementEvent,
        settings: movement_settings_t__bindgen_ty_1,
        // event: movement_event_t,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    ) -> bool;
    fn face_resign(
        &mut self,
        settings: movement_settings_t__bindgen_ty_1,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    );
    fn face_wants_background_task(
        &mut self,
        _settings: movement_settings_t__bindgen_ty_1,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    ) -> bool {
        false
    }

    /// After initial setup, pass a reference to the allocated context, so it can be initialized in a static
    ///
    /// This will only be run once after `face_initial_setup` is called
    fn face_post_initial_setup(&'static self) {
        info!("Called face_post_initial_setup");
    }
}
