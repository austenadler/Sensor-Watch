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
    watch_display_string, watch_utility_offset_timestamp, EventType, MovementEvent,
};

const NUM_TIMERS: usize = 5;
const NUM_TIMER_PRESETS: usize = 3;
const DEFAULT_TIMER_PRESETS: &[usize; NUM_TIMER_PRESETS] = &[30, 60, 90];

#[derive(Debug)]
enum FaceState {
    AllTimers,
    Timer(usize),
    EditPresets(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum TimerState {
    Ready,
    Started,
    Paused,
}

// impl TimerState {
//     /// Toggle the state of the timer
//     ///
//     /// Start the timer if stopped or paused,
//     fn toggle(&self) -> Self {match self{
//         Self::Ready => Self::Started,
//         Self::Started => Self::Paused,
//         Self::Paused => Self::Started,
//     }}
// }

impl Default for TimerState {
    fn default() -> Self {
        Self::Ready
    }
}

#[derive(Debug)]
struct Timer {
    idx: u8,
    state: TimerState,
    timer_preset_idx: u8,
    // time_remaining: Option<watch_date_time__bindgen_ty_1>,
}

impl Timer {
    fn new(idx: usize) -> Self {
        Self {
            idx: idx as u8,
            state: TimerState::default(),
            timer_preset_idx: 0,
        }
    }

    fn advance_timer_preset(&mut self) {
        self.timer_preset_idx = self.timer_preset_idx.saturating_add(1) % NUM_TIMER_PRESETS as u8;
    }

    fn draw(&self, ctx: &Context) {
        // if self.is_flashing && self.flash_toggle {
        //         unsafe {
        //             watch_clear_display();
        //         }
        //         return;
        // }

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
    all_timers_idx: Option<u8>,
    // is_flashing: bool,
    // flashing_toggle: bool,
}

impl Context {
    /// Idempotent logic to handle any changes in timer start/stop state
    fn refresh_running_status(&mut self) {
        {
            // Update the number of running timers
            self.num_running_timers = self
                .timers
                .iter()
                .filter(|t| t.state == TimerState::Started)
                .count() as u8;

            self.display_indicator_state
                .bell
                .set(self.num_running_timers > 0);
        }
    }

    fn advance_state(&mut self) {
        self.face_state = match self.face_state {
            FaceState::AllTimers => FaceState::Timer(0),
            FaceState::Timer(n)
                if self.num_running_timers > 0 && (n.saturating_add(1)) as usize == NUM_TIMERS =>
            {
                // Whenever we switch to all timers state, update the nearest timer

                self.all_timers_idx = self.nearest_timer().map(|t| t.idx);
                FaceState::AllTimers
            }
            FaceState::Timer(n) => FaceState::Timer((n + 1) % NUM_TIMERS),
            FaceState::EditPresets(n) => FaceState::Timer(n),
        };
    }

    /// Return the idx of the next timer that will go off
    fn nearest_timer(&self) -> Option<&Timer> {
        // TODO: This just returns the first timer
        self.timers.iter().find(|t| t.state == TimerState::Started)
    }

    fn draw_all_timers_face(&mut self) {
        // self.all_timers_idx = .map(|t| t.idx);
        let Some(first_running_timer) = self.all_timers_idx.map(|idx| &self.timers[idx as usize])
        // .as_ref()
        // .or_else(|| self.nearest_timer())
        else {
            // We don't want to be on the AT face if there are no running timers
            self.advance_state();
            return;
        };

        self.display_indicator_state.signal.set(false);
        unsafe {
            watch_display_string(cstr!("AT        ").as_ptr().cast_mut(), 0);
        }
        sensor_watch_sys::watch_display_u8(first_running_timer.idx + 1, false, 2);

        // let mut header_buf = [0x0; 10 + 1];
        // header_buf[0] = b'A';
        // header_buf[1] = b'T';
        // header_buf[2] = b' ';
        // sensor_watch_sys::write_u8_chars(
        //     &mut header_buf[3..=4],
        //     self.num_running_timers as u8,
        //     false,
        // );
        // header_buf[4] = 0x0;

        // unsafe {
        //     watch_display_string(
        //         CStr::from_bytes_with_nul_unchecked(&header_buf)
        //             .as_ptr()
        //             .cast_mut(),
        //         0,
        //     );
        // }
    }

    fn draw_timer_face(&mut self, timer_n: usize) {
        let timer = &self.timers[timer_n];
        timer.draw(&self);
        self.display_indicator_state
            .signal
            .set(timer.state == TimerState::Started);
    }

    fn draw_edit_face(&mut self) {
        unsafe {
            watch_display_string(cstr!("EDIT      ").as_ptr().cast_mut(), 0);
        }
    }

    fn draw(&mut self) {
        match self.face_state {
            FaceState::AllTimers => self.draw_all_timers_face(),
            FaceState::Timer(timer_n) => self.draw_timer_face(timer_n),
            FaceState::EditPresets(_) => self.draw_edit_face(),
        }
    }

    /* ======= Timer functions ======= */
    fn update_timer_state(&mut self, timer: &mut Timer, new_state: TimerState) {
        match (&timer.state, &new_state) {
            (TimerState::Ready, TimerState::Ready)
            | (TimerState::Started, TimerState::Started)
            | (TimerState::Paused, TimerState::Paused)
            | (TimerState::Ready, TimerState::Paused) => {
                // Impossible transition
            }
            (TimerState::Ready, TimerState::Started) => todo!(),
            (TimerState::Started, TimerState::Ready) => todo!(),
            (TimerState::Started, TimerState::Paused) => todo!(),
            (TimerState::Paused, TimerState::Ready) => todo!(),
            (TimerState::Paused, TimerState::Started) => todo!(),
        }

        unsafe {
            watch_utility_offset_timestamp();
        }

        // match timer.state {
        //     TimerState::Ready => {
        //         let remaining_time = self.timer_presets[timer.timer_preset_idx as usize];
        //     },
        //     TimerState::Started => todo!(),
        //     TimerState::Paused => todo!(),
        // };

        // fn selected_preset_time(&self) -> usize {
        //     self.timer_preset_idx % NUM_TIMER_PRESETS as u8;
        // }
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
            display_indicator_state: DisplayIndicatorState::new(),
            all_timers_idx: None,
            // is_flashing: false,
            // flashing_toggle: false,
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
        info!("Event: {event:?}");

        match event.event_type {
            EventType::Tick if event.subsecond == 0 => {
                // Update all running timers
                self.draw();
            }
            EventType::Tick => {
                // Update ui flashing tick
                self.draw();
            }
            EventType::Activate => {
                // Clear whole screen
                unsafe {
                    watch_clear_display();
                }
                // Keep state of cleared screen
                self.display_indicator_state = DisplayIndicatorState::new();
                // Refresh our running timer status
                self.refresh_running_status();
                self.face_state = if self.num_running_timers == 0 {
                    FaceState::Timer(0)
                } else {
                    FaceState::AllTimers
                };
                self.draw();
            }
            /* ======= Alarm Button ======= */
            EventType::AlarmButtonUp => {
                match self.face_state {
                    FaceState::AllTimers => {
                        // Next running timer
                    }
                    FaceState::Timer(timer_n) => {
                        let timer = &self.timers[timer_n];
                        match timer.state {
                            TimerState::Ready => {
                                // Next timer preset
                                self.timers[timer_n].advance_timer_preset();
                            }
                            TimerState::Started => {
                                // Add 30 seconds
                            }
                            TimerState::Paused => {
                                // ?
                            }
                        }
                    }
                    FaceState::EditPresets(_) => {}
                }
                self.draw();
            }
            EventType::AlarmLongPress => {
                match self.face_state {
                    FaceState::AllTimers => {
                        // Jump to timer at selected index
                    }
                    FaceState::Timer(timer_n) => {
                        let timer = &mut self.timers[timer_n];
                        match timer.state {
                            TimerState::Ready => {
                                // Start timer
                                timer.state = TimerState::Started;
                            }
                            TimerState::Started => {
                                // Pause timer
                                timer.state = TimerState::Paused;
                            }
                            TimerState::Paused => {
                                // Resume timer
                                timer.state = TimerState::Started;
                            }
                        }
                        self.refresh_running_status();
                    }
                    FaceState::EditPresets(_) => {}
                }
                self.draw();
            }
            /* ======= Light Button ======= */
            EventType::LightButtonUp => {
                match self.face_state {
                    FaceState::AllTimers | FaceState::Timer(_) => {
                        // Next timer
                        self.advance_state();
                    }
                    FaceState::EditPresets(_) => {}
                }
                self.draw();
            }
            EventType::LightLongPress => {
                match self.face_state {
                    FaceState::AllTimers => {
                        // Reset the currently selected timer
                    }
                    FaceState::Timer(timer_n) => {
                        let timer = &self.timers[timer_n];
                        match timer.state {
                            TimerState::Ready => {
                                // Switch to edit mode
                                self.face_state = FaceState::EditPresets(timer_n);
                            }
                            TimerState::Started | TimerState::Paused => {
                                // Reset timer
                            }
                        }
                    }
                    FaceState::EditPresets(_) => {
                        // Switch out of edit mode
                        self.advance_state();
                    }
                }
                self.draw();
            }
            /* ======= End of Buttons ======= */
            EventType::LightButtonDown => {
                // Keep empty so the light is never illuminated
                // Don't cook in the dark
            }
            _ => unsafe {
                movement_default_loop_handler(event.into(), &mut (settings.into()));
            },
        }

        false
    }

    fn face_resign(&mut self, _settings: movement_settings_t__bindgen_ty_1) {
        info!("In face_resign {self:?}");

        core::mem::replace(
            &mut self.display_indicator_state,
            DisplayIndicatorState::new(),
        )
        .resign();

        // self.display_indicator_state.resign();
    }
}
