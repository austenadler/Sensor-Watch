use crate::{expose_face, face::WatchFace};
use sensor_watch_sys::{info, movement_settings_t__bindgen_ty_1, MovementEvent};

#[derive(Debug)]
struct Context;

expose_face!(Context, hi_rust);

impl WatchFace for Context {
    fn face_initial_setup(
        _settings: &movement_settings_t__bindgen_ty_1,
        watch_face_index: cty::uint8_t,
    ) -> Self {
        info!("In face_initial_setup ({watch_face_index})");
        Self {}
    }

    fn face_activate(&mut self, _settings: &movement_settings_t__bindgen_ty_1) {
        info!("In face_activate {self:?}");
    }

    fn face_loop(
        &mut self,
        event: sensor_watch_sys::MovementEvent,
        _settings: &movement_settings_t__bindgen_ty_1,
    ) -> bool {
        info!("In face_loop {self:?} ({event:?})");
        false
    }

    fn face_resign(&mut self, _settings: &movement_settings_t__bindgen_ty_1) {
        info!("In face_resign {self:?}");
    }
}
