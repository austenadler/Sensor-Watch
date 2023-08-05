use crate::face::WatchFace;
use cstr::cstr;
use cty::uint8_t;
use derive::WatchFace;
use sensor_watch_sys::watch_date_time__bindgen_ty_1;
use sensor_watch_sys::{
    info, movement_default_loop_handler, movement_settings_t, movement_settings_t__bindgen_ty_1,
    watch_display_string, EventType, MovementEvent,
};

const NUM_TIMERS: usize = 5;
const DEFAULT_TIMER_TIMES: &[usize; 2] = &[30, 60];

#[derive(Debug)]
enum FaceState {
    AllTimers,
    Timer(u8),
}

#[derive(Debug, Default)]
struct Timer {
    started: bool,
    end_time: Option<watch_date_time__bindgen_ty_1>,
}

#[derive(Debug, WatchFace)]
#[watch_face(kitchen_timer)]
// TODO: Is it unsafe to libc::malloc a non-repr(C) object, even if it's only accessed within rust?
// #[repr(C)]
struct Context {
    face_state: FaceState,
    watch_face_index: uint8_t,
    timers: [Timer; NUM_TIMERS],
}

impl Context {
    fn advance_state(&mut self) {
        self.face_state = match self.face_state {
            FaceState::AllTimers => FaceState::Timer(0_u8),
            FaceState::Timer(n) if (n.saturating_add(1) ) as usize == NUM_TIMERS => FaceState::AllTimers,
            FaceState::Timer(n) => FaceState::Timer(n+1_u8),
        };
    }

    fn draw(&self) {
        match self.face_state {
            FaceState::AllTimers => unsafe {
                watch_display_string(cstr!("AT  ").as_ptr().cast_mut(), 0);
            },
            FaceState::Timer(timer_n) => {
                let number: [i8;2] = [(b'0' + timer_n).try_into().unwrap(), 0x0];
                unsafe {
                watch_display_string(number.as_ptr().cast_mut(), 0);
                watch_display_string(cstr!("T  ").as_ptr().cast_mut(), 1);
            }
            },
        }
    }
}

impl WatchFace for Context {
    fn face_initial_setup(
        _settings: movement_settings_t__bindgen_ty_1,
        watch_face_index: uint8_t,
    ) -> Self {
        info!("In face_initial_setup ({watch_face_index})");
        Self {
            face_state: FaceState::AllTimers,
            watch_face_index,
            timers: Default::default(),
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
            EventType::Tick => {}
            EventType::Activate => {
                self.draw();
            }
            EventType::LightButtonDown => {
                info!("Advanced state");
                self.advance_state();
                info!("{:?}", self.face_state);

                self.draw();
            }
            _ => {
                unsafe {
                    movement_default_loop_handler(event.into(), &mut (settings.into()));
                }
            }
        }

        false
    }

    fn face_resign(&mut self, _settings: movement_settings_t__bindgen_ty_1) {
        info!("In face_resign {self:?}");
        // self.last_viewed = false;
    }
}
