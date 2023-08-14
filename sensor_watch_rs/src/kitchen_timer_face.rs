use core::{array, ffi::CStr};

use crate::face::WatchFace;
use cstr::cstr;
use cty::uint8_t;
use derive::WatchFace;
use sensor_watch_sys::{
    display::indicator::DisplayIndicatorState, info, movement_default_loop_handler,
    movement_settings_t, movement_settings_t__bindgen_ty_1, watch_clear_display,
    watch_date_time__bindgen_ty_1, watch_display_string, watch_display_u8, watch_set_colon,
    watch_utility_offset_timestamp, write_u8_chars, EventType, MovementEvent,
    WatchIndicatorSegment,
};

const NUM_TIMERS: usize = 5;
const NUM_TIMER_PRESETS: usize = 3;
const DEFAULT_TIMER_PRESETS: &[TimeEntry; NUM_TIMER_PRESETS] = &[
    TimeEntry {
        hours: 0,
        minutes: 0,
        seconds: 30,
    },
    TimeEntry {
        hours: 0,
        minutes: 1,
        seconds: 0,
    },
    TimeEntry {
        hours: 0,
        minutes: 1,
        seconds: 30,
    },
];

use sensor_watch_sys::time::WatchDateTime;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TimeEntry {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl TimeEntry {
    pub fn tick(&mut self) -> bool {
        // self.seconds = self.seconds.saturating_sub(1) ;
        // TODO: This might not be super efficient
        *self = Self::from_seconds(self.as_seconds().saturating_sub(1));
        self.is_zero()
    }

    pub fn watch_display(&self) {
        let mut buf = [0x0; 6 + 1];

        write_u8_chars(&mut buf[0..=1], self.hours, true);
        write_u8_chars(&mut buf[2..=3], self.minutes, true);
        write_u8_chars(&mut buf[4..=5], self.seconds, true);
        // Just in case the write_u8_chars api changes, ensure the last element is zero
        buf[6] = 0x0;

        // buf is already zeroed, so we don't have to worry about null termination
        let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(&buf) };

        // Now, actually write it
        unsafe {
            watch_display_string(cstr.as_ptr().cast_mut(), 4);
            watch_set_colon();
        }
    }

    pub fn is_zero(&self) -> bool {
        self == &Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }

    /// ```rust
    /// use crate::TimeEntry;
    /// assert_eq!(TimeEntry::from_seconds(0), TimeEntry {hours:0, minutes:0, seconds:0});
    /// assert_eq!(TimeEntry::from_seconds(32), TimeEntry {hours:0, minutes:0, seconds:32});
    /// assert_eq!(TimeEntry::from_seconds(60), TimeEntry {hours:0, minutes:1, seconds:0});
    /// assert_eq!(TimeEntry::from_seconds(59), TimeEntry {hours:0, minutes:0, seconds:59});
    /// assert_eq!(TimeEntry::from_seconds(61), TimeEntry {hours:0, minutes:1, seconds:1});
    /// assert_eq!(TimeEntry::from_seconds(3599), TimeEntry {hours:0, minutes:59, seconds:59});
    /// assert_eq!(TimeEntry::from_seconds(3600), TimeEntry {hours:1, minutes:0, seconds:0});
    /// assert_eq!(TimeEntry::from_seconds(3601), TimeEntry {hours:1, minutes:0, seconds:1});
    /// assert_eq!(TimeEntry::from_seconds(3660), TimeEntry {hours:1, minutes:1, seconds:0});
    /// ```
    pub fn from_seconds(mut input: u32) -> Self {
        let seconds = (input % 60) as u8;
        input /= 60;
        let minutes = (input % 60) as u8;
        input /= 60;
        let hours = (input % 60) as u8;

        Self {
            hours,
            minutes,
            seconds,
        }
    }

    /// ```rust
    /// use crate::TimeEntry;
    /// assert_eq!(TimeEntry {hours:0, minutes:0, seconds:0}.as_seconds(), 0);
    /// assert_eq!(TimeEntry {hours:0, minutes:0, seconds:32}.as_seconds(), 32);
    /// assert_eq!(TimeEntry {hours:0, minutes:1, seconds:0}.as_seconds(), 60);
    /// assert_eq!(TimeEntry {hours:0, minutes:0, seconds:59}.as_seconds(), 59);
    /// assert_eq!(TimeEntry {hours:0, minutes:1, seconds:1}.as_seconds(), 61);
    /// assert_eq!(TimeEntry {hours:0, minutes:59, seconds:59}.as_seconds(), 3599);
    /// assert_eq!(TimeEntry {hours:1, minutes:0, seconds:0}.as_seconds(), 3600);
    /// assert_eq!(TimeEntry {hours:1, minutes:0, seconds:1}.as_seconds(), 3601);
    /// assert_eq!(TimeEntry {hours:1, minutes:1, seconds:0}.as_seconds(), 3660);
    /// ```
    pub fn as_seconds(&self) -> u32 {
        self.seconds as u32 + self.minutes as u32 * 60 + self.hours as u32 * 60 * 60
    }
}

#[derive(Debug)]
enum FaceState {
    AllTimers,
    Timer(usize),
    EditPresets(usize),
}

#[derive(Debug)]
enum TimerState {
    Ready,
    /// The timer has started, and this will be counted down whenever the watch ticks
    Started {
        time_remaining: TimeEntry,
    },
    /// The timer is paused, and there are u32 more seconds to wait
    Paused {
        time_remaining: TimeEntry,
    },
}

impl TimerState {
    fn is_started(&self) -> bool {
        match self {
            Self::Started { .. } => true,
            _ => false,
        }
    }
}

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

    // fn draw(&self, ctx: &Context) {
    // }

    fn draw_header(&self) {
        let mut header_buf = [0x0; 4 + 1];

        // TODO: Decide if I want the preset number to show up at the top right

        {
            // Draws ` T x` where `x` is the timer number
            header_buf[0] = b'T';
            header_buf[1] = b' ';
            header_buf[2] = b' ';
            sensor_watch_sys::write_u8_chars(&mut header_buf[3..=3], self.idx as u8 + 1, false);
        }

        // {
        //     // Draws `xT y` where `x` is the timer number, and `y` is the preset number
        //     sensor_watch_sys::write_u8_chars(&mut header_buf[0..=0], self.idx as u8 + 1, false);
        //     header_buf[1] = b'T';
        //     header_buf[2] = b' ';
        //     sensor_watch_sys::write_u8_chars(
        //         &mut header_buf[3..=3],
        //         self.timer_preset_idx + 1,
        //         false,
        //     );
        // }

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
    timer_presets: [TimeEntry; NUM_TIMER_PRESETS],
    display_indicator_state: DisplayIndicatorState,
    all_timers_idx: Option<u8>,
    blink_toggle: bool,
    // is_flashing: bool,
    // flashing_toggle: bool,
}

impl Context {
    /// Idempotent logic to handle any changes in timer start/stop state
    fn refresh_running_status(&mut self) {
        {
            // Update the number of running timers
            self.num_running_timers =
                self.timers.iter().filter(|t| t.state.is_started()).count() as u8;

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
        self.timers.iter().find(|t| t.state.is_started())
    }

    fn draw_all_timers_face(&mut self) {
        info!("Drawing all timers face");
        // self.all_timers_idx = .map(|t| t.idx);
        let Some(first_running_timer) = self.all_timers_idx.map(|idx| &self.timers[idx as usize])
        // .as_ref()
        // .or_else(|| self.nearest_timer())
        else {
            // We don't want to be on the AT face if there are no running timers
            // TODO: This shouldn't be in the draw function
            self.advance_state();
            self.draw();
            return;
        };

        self.display_indicator_state.signal.set(false);
        unsafe {
            watch_display_string(cstr!("AT        ").as_ptr().cast_mut(), 0);
        }
        watch_display_u8(first_running_timer.idx + 1, false, 2);
    }

    fn draw_timer_face(&mut self, timer_n: usize) {
        info!("Drawing timer face for {timer_n}");
        let timer = &self.timers[timer_n];
        self.display_indicator_state
            .signal
            .set(timer.state.is_started());

        // if self.is_flashing && self.flash_toggle {
        //         unsafe {
        //             watch_clear_display();
        //         }
        //         return;
        // }

        timer.draw_header();
        // Draw the time display
        match &timer.state {
            TimerState::Ready => {
                self.display_indicator_state.tick_frequency.set(1);
                // Draw the preset that's ready to go
                self.timer_presets[timer.timer_preset_idx as usize].watch_display()
            }
            TimerState::Started { time_remaining } => {
                self.display_indicator_state.tick_frequency.set(1);
                time_remaining.watch_display();
            }
            TimerState::Paused { time_remaining } => {
                // We want to blink more frequently
                self.display_indicator_state.tick_frequency.set(2);

                // TODO: Flash here
                if self.blink_toggle {
                    time_remaining.watch_display();
                    // Blank the time
                    unsafe {
                        watch_display_string(cstr!("      ").as_ptr().cast_mut(), 4);
                        watch_set_colon();
                    }
                } else {
                    time_remaining.watch_display();
                }
            }
        }
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
    fn start_timer(&mut self, timer_idx: usize) {
        info!("Starting timer {timer_idx}");

        let timer = &mut self.timers[timer_idx];

        match &timer.state {
            TimerState::Ready => {
                let time_to_wait = &self.timer_presets[timer.timer_preset_idx as usize];
                // watch_utility_offset_timestamp
                // let now = WatchDateTime::now().timestamp_utc();
                // let target_time = now + time_to_wait.as_seconds();
                // info!("Got difference of: {:?}", time_to_wait);

                // unsafe {
                //     watch_utility_offset_timestamp(
                //         now.as_ref(),
                //         time_to_wait.hours as i8,
                //         time_to_wait.minutes as i8,
                //         time_to_wait.seconds as i8,
                //     )
                // }
                info!("Timer is ready. Wait time: {time_to_wait:?}");

                timer.state = TimerState::Started {
                    time_remaining: time_to_wait.clone(),
                    // time_remaining: time_to_wait,
                    // time_remaining:
                };
            }
            TimerState::Started { time_remaining: _ } => {
                // We can't start a started timer
                return;
            }
            TimerState::Paused { time_remaining } => {
                timer.state = TimerState::Started {
                    time_remaining: time_remaining.clone(),
                }
            }
        }
    }

    fn tick_all_timers(&mut self) {
        for t in &mut self.timers {
            if let TimerState::Started {
                ref mut time_remaining,
            } = t.state
            {
                time_remaining.tick();
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
            display_indicator_state: DisplayIndicatorState::new(),
            all_timers_idx: None,
            blink_toggle: false,
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
        if let EventType::Tick = event.event_type {
            self.blink_toggle = !self.blink_toggle;
        }

        match event.event_type {
            EventType::Tick if event.subsecond == 0 => {
                // Update all running timers
                self.tick_all_timers();
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
                info!("Initialized to {:?}", self.face_state);
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
                        match &timer.state {
                            TimerState::Ready => {
                                // Next timer preset
                                self.timers[timer_n].advance_timer_preset();
                            }
                            TimerState::Started { time_remaining: _ } => {
                                // Add 30 seconds
                            }
                            TimerState::Paused { time_remaining: _ } => {
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
                        match &timer.state {
                            TimerState::Ready => {
                                // Start timer
                                self.start_timer(timer_n);
                            }
                            TimerState::Started { time_remaining } => {
                                // Pause timer
                                timer.state = TimerState::Paused {
                                    time_remaining: time_remaining.clone(),
                                };
                            }
                            TimerState::Paused { time_remaining: _ } => {
                                // Resume timer
                                self.start_timer(timer_n);
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
                        let timer = &mut self.timers[timer_n];
                        match &timer.state {
                            TimerState::Ready => {
                                // Switch to edit mode
                                self.face_state = FaceState::EditPresets(timer_n);
                            }
                            TimerState::Started { time_remaining: _ }
                            | TimerState::Paused { time_remaining: _ } => {
                                // Reset timer
                                timer.state = TimerState::Ready;
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
    }
}
