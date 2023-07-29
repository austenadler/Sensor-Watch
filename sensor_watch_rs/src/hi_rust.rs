use crate::face::WatchFace;
use cstr::cstr;
use cty::uint8_t;
use derive::WatchFace;
use sensor_watch_sys::{
    info, movement_default_loop_handler, movement_settings_t, movement_settings_t__bindgen_ty_1,
    watch_display_string, EventType, MovementEvent,
};

#[derive(Debug, WatchFace)]
#[watch_face(hi_rust)]
// TODO: Is it unsafe to libc::malloc a non-repr(C) object, even if it's only accessed within rust?
// #[repr(C)]
struct Context {
    last_viewed: bool,
    watch_face_index: uint8_t,
}

impl WatchFace for Context {
    fn face_initial_setup(
        _settings: movement_settings_t__bindgen_ty_1,
        watch_face_index: uint8_t,
    ) -> Self {
        info!("In face_initial_setup ({watch_face_index})");
        Self {
            last_viewed: false,
            watch_face_index,
        }
    }

    fn face_activate(&mut self, _settings: movement_settings_t__bindgen_ty_1) {
        info!("In face_activate {self:?}");
    }

    fn face_loop(
        &mut self,
        event: MovementEvent,
        settings: movement_settings_t__bindgen_ty_1,
    ) -> bool {
        info!("In face_loop {self:?} ({event:?})");

        match event.event_type {
            EventType::Activate | EventType::Tick => {
                if self.last_viewed {
                    unsafe { watch_display_string(cstr!("HI  DANI").as_ptr().cast_mut(), 0) };
                } else {
                    unsafe { watch_display_string(cstr!("        ").as_ptr().cast_mut(), 0) };
                }
                self.last_viewed = !self.last_viewed;
            }
            _ => {}
        }

        unsafe {
            movement_default_loop_handler(event.into(), &mut (settings.into()));
        }

        false
    }

    fn face_resign(&mut self, _settings: movement_settings_t__bindgen_ty_1) {
        info!("In face_resign {self:?}");
        self.last_viewed = false;
    }
}
