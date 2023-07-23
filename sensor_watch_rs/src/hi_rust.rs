use crate::{expose_face, face::WatchFace};
use cstr::cstr;
use cty::uint8_t;
use sensor_watch_sys::{info, movement_settings_t__bindgen_ty_1, MovementEvent, watch_display_string};

#[derive(Debug)]
#[repr(C)]
struct Context {
    last_viewed: bool,
    watch_face_index: uint8_t,
}

expose_face!(Context, hi_rust);

impl WatchFace for Context {
    fn face_initial_setup(
        _settings: &movement_settings_t__bindgen_ty_1,
        watch_face_index: uint8_t,
    ) -> Self {
        info!("In face_initial_setup ({watch_face_index})");
        Self {
            last_viewed: false,
            watch_face_index,
        }
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
        if self.last_viewed {
            unsafe { watch_display_string(cstr!("HI  DANI").as_ptr().cast_mut(), 0) };
        } else {
            unsafe { watch_display_string(cstr!("        ").as_ptr().cast_mut(), 0) };
        }
        self.last_viewed = !self.last_viewed;
        false
    }

    fn face_resign(&mut self, _settings: &movement_settings_t__bindgen_ty_1) {
        info!("In face_resign {self:?}");
    }
}
