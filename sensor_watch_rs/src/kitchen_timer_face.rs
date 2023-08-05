use core::array;
use core::ffi::CStr;

use crate::face::WatchFace;
use cstr::cstr;
use cty::uint8_t;
use derive::WatchFace;
use sensor_watch_sys::{
    display::indicator::DisplayIndicatorState, watch_clear_display, watch_date_time__bindgen_ty_1,
    WatchIndicatorSegment,
};
use sensor_watch_sys::{
    info, movement_default_loop_handler, movement_settings_t, movement_settings_t__bindgen_ty_1,
    watch_display_string, EventType, MovementEvent,
};

const NUM_TIMERS: usize = 5;
const NUM_TIMER_PRESETS: usize = 3;
const DEFAULT_TIMER_PRESETS: &[usize; NUM_TIMER_PRESETS] = &[30, 60, 90];

#[derive(Debug)]
enum FaceState {
    AllTimers,
    Timer(usize),
}

#[derive(Debug)]
struct Timer {
    idx: u8,
    started: bool,
    timer_preset_idx: u8,
    // time_remaining: Option<watch_date_time__bindgen_ty_1>,
}

impl Timer {
    fn new(idx: usize) -> Self {
        Self {
            idx: idx as u8,
            started: false,
            timer_preset_idx: 0,
        }
    }

    fn advance_timer_preset(&mut self) {
        self.timer_preset_idx = self.timer_preset_idx.saturating_add(1) % NUM_TIMER_PRESETS as u8;
    }

    fn draw(&self, ctx: &Context) {
        self.draw_header();

        // For debugging
        sensor_watch_sys::watch_display_u8(
            ctx.timer_presets[self.timer_preset_idx as usize] as u8,
            true,
            6,
        );
    }

    fn draw_header(&self) {
        let mut header_buf = [0x0; 4 + 1];
        sensor_watch_sys::write_u8_chars(&mut header_buf[0..=0], self.idx as u8 + 1, false);
        header_buf[1] = b'T';
        header_buf[2] = b' ';
        sensor_watch_sys::write_u8_chars(&mut header_buf[3..=3], self.timer_preset_idx + 1, false);
        // Just to be safe
        header_buf[4] = 0x0;

        unsafe {
            watch_display_string(
                CStr::from_bytes_with_nul_unchecked(&header_buf)
                    .as_ptr()
                    .cast_mut(),
                0,
            );
        }
    }
}

#[derive(Debug, WatchFace)]
#[watch_face(kitchen_timer)]
// TODO: Is it unsafe to libc::malloc a non-repr(C) object, even if it's only accessed within rust?
// #[repr(C)]
struct Context {
    face_state: FaceState,
    _watch_face_index: uint8_t,
    timers: [Timer; NUM_TIMERS],
    num_running_timers: u8,
    timer_presets: [usize; NUM_TIMER_PRESETS],
    display_indicator_state: DisplayIndicatorState,
}

impl Context {
    /// Idempotent logic to handle any changes in timer start/stop state
    fn refresh_running_status(&mut self, force_update_display: bool) {
        {
            // Update the number of running timers
            let old_has_running_timers = self.num_running_timers > 0;
            self.num_running_timers = self.timers.iter().filter(|t| t.started).count() as u8;
            let has_running_timers = self.num_running_timers > 0;

            if old_has_running_timers != has_running_timers || force_update_display {
                // Whatever we thought before is no longer true. Need to update indicators
                self.display_indicator_state.set_bell(has_running_timers);
            }
        }
    }

    fn advance_state(&mut self) {
        // TODO: If there are no running timers, should we show the first timer?
        self.face_state = match self.face_state {
            FaceState::AllTimers => FaceState::Timer(0),
            FaceState::Timer(n) if (n.saturating_add(1)) as usize == NUM_TIMERS => {
                FaceState::AllTimers
            }
            FaceState::Timer(n) => FaceState::Timer(n + 1),
        };
    }

    fn draw(&mut self) {
        match self.face_state {
            FaceState::AllTimers => {
                sensor_watch_sys::watch_display_u8(self.num_running_timers as u8, false, 2);
                unsafe {
                    watch_display_string(cstr!("AT  ").as_ptr().cast_mut(), 0);
                }

                let mut header_buf = [0x0; 4 + 1];
                header_buf[0] = b'A';
                header_buf[1] = b'T';
                header_buf[2] = b' ';
                sensor_watch_sys::write_u8_chars(
                    &mut header_buf[3..=4],
                    self.num_running_timers as u8,
                    false,
                );
                header_buf[4] = 0x0;

                unsafe {
                    watch_display_string(
                        CStr::from_bytes_with_nul_unchecked(&header_buf)
                            .as_ptr()
                            .cast_mut(),
                        0,
                    );
                }
            }
            FaceState::Timer(timer_n) => {
                let timer = &self.timers[timer_n];
                timer.draw(&self);
                self.display_indicator_state.set_signal(timer.started);
            }
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
            _watch_face_index: watch_face_index,
            timers: array::from_fn(Timer::new),
            num_running_timers: 0,
            timer_presets: DEFAULT_TIMER_PRESETS.clone(),
            display_indicator_state: DisplayIndicatorState::default(),
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
                unsafe {
                    watch_clear_display();
                }
                self.display_indicator_state = DisplayIndicatorState::default();
                // Refresh our running timer status
                self.refresh_running_status(true);
                self.face_state = FaceState::AllTimers;
                self.draw();
            }
            EventType::LightButtonUp => {
                self.advance_state();
                self.draw();
            }
            EventType::AlarmButtonUp => {
                if let FaceState::Timer(timer_n) = self.face_state {
                    self.timers[timer_n].advance_timer_preset();
                    self.draw();
                }
            }
            EventType::AlarmLongPress => {
                if let FaceState::Timer(timer_n) = self.face_state {
                    self.timers[timer_n].started = !self.timers[timer_n].started;
                    self.refresh_running_status(false);
                self.draw();
                }
            }
            EventType::LightButtonDown => {}
            _ => unsafe {
                movement_default_loop_handler(event.into(), &mut (settings.into()));
            },
        }

        false
    }

    fn face_resign(&mut self, _settings: movement_settings_t__bindgen_ty_1) {
        info!("In face_resign {self:?}");
        // self.last_viewed = false;
    }
}
