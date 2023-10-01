use core::{array, cmp::Ordering, ffi::CStr};

use cstr::cstr;
use cty::uint8_t;
use sensor_watch_rs::{
    derive::WatchFace,
    time::WatchDateTime,
    display::DisplayIndicatorState,
    face::WatchFace,
    info,
    EventType, MovementEvent, write_u8_chars,watch_display_u8, 
time::TimeEntry,    sys::{
        movement_cancel_background_task, movement_cancel_background_task_for_face,
        movement_default_loop_handler, movement_schedule_background_task_for_face,
        movement_settings_t, movement_settings_t__bindgen_ty_1, 
        watch_buzzer_play_sequence, watch_clear_colon, watch_clear_display,
        watch_date_time__bindgen_ty_1, watch_display_string, watch_set_colon,
        watch_utility_date_time_from_unix_time, watch_utility_offset_timestamp,
        BuzzerNote,  WatchIndicatorSegment,
    },
};

// TODO: This must be static because the callback to buzzer needs to be a function with no parameters
/// A reference to the initialized kitchen timer context
static mut STATE_REF: Option<*mut Context> = None;

static BEEPS: &[i8] = &[
    BuzzerNote::BUZZER_NOTE_C8.0 as i8,
    3,
    BuzzerNote::BUZZER_NOTE_REST.0 as i8,
    3,
    -2,
    2,
    BuzzerNote::BUZZER_NOTE_C8.0 as i8,
    5,
    BuzzerNote::BUZZER_NOTE_REST.0 as i8,
    25,
    0,
];

const NUM_TIMERS: usize = 5;
const NUM_TIMER_PRESETS: usize = 3;
const DEFAULT_TIMER_PRESETS: &[TimeEntry; NUM_TIMER_PRESETS] = &[
    TimeEntry {
        hours: 0,
        minutes: 0,
        seconds: 5,
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


extern "C" fn callback() {
    info!("Callback has been called");
}

// fn register_callback(cb: extern fn(i32)) -> i32;

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

    /// The number of seconds remaining until this timer ends
    fn seconds_remaining(&self) -> u32 {
        match &self.state {
            TimerState::Ready => 0,
            TimerState::Paused { time_remaining } | TimerState::Started { time_remaining } => {
                time_remaining.as_seconds()
            }
        }
    }

    fn draw_header(&self) {
        // let mut header_buf = [0x0; 4 + 1];
        let mut header_buf: [u8; 4 + 1] = *b"T  _\0";

        sensor_watch_rs::write_u8_chars(&mut header_buf[3..=3], self.idx as u8 + 1, false);

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
    watch_face_index: uint8_t,
    timers: [Timer; NUM_TIMERS],
    num_running_timers: u8,
    timer_presets: [TimeEntry; NUM_TIMER_PRESETS],
    display_indicator_state: DisplayIndicatorState,
    // The timer index that the all_timers is displaying
    all_timers_idx: Option<u8>,
    blink_toggle: bool,
    /// The number of seconds until the next alarm goes off, and the index of the timer
    next_alarm: Option<(u32, u8)>,
    // /// The next timer that will go off
    // next_timer_alarm: Option<u8>,
    // is_flashing: bool,
    // flashing_toggle: bool,
}

impl Context {
    /// Handle any changes in timer start/stop state
    fn refresh_running_status(&mut self) {
        info!("Refreshing running status");

        // Update the number of running timers and bell icon
        {
            self.num_running_timers =
                self.timers.iter().filter(|t| t.state.is_started()).count() as u8;

            self.display_indicator_state
                .bell
                .set(self.num_running_timers > 0);
        }

        // The next timer that will be going off
        let previous_alarm = self.next_alarm;
        self.next_alarm = self.nearest_timer().map(|t| (t.seconds_remaining(), t.idx));

        match (previous_alarm, self.next_alarm) {
            // Stopping a background alert
            (Some(_), None) => {
                // We had a timer, and we don't have one anymore
                // Cancel whatever background task we had
                info!("Cancelling background task");
                unsafe { movement_cancel_background_task_for_face(self.watch_face_index) };
            }

            // Setting a background alert
            (None, Some(next)) => {
                // We didn't have a timer before, but we do now
                info!("Got: (None, Some(_))");

                let time = WatchDateTime::now() + next.0;
                time.schedule_background_task_for_face(self.watch_face_index);
            }
            (Some(current), Some(next)) if current != next => {
                // We have a new "next" timer. This happens when a newly started timer will finish before the old one
                info!("Got: (Some(_) != Some(_))");

                let time = WatchDateTime::now() + next.0;
                time.schedule_background_task_for_face(self.watch_face_index);
            }
            // No timers are started, or the next timer time is the same
            (None, None) | (Some(_), Some(_)) => {}
        }

        info!("Set next_alarm to {:?}", self.next_alarm);
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

    /// Advance the state of the all timers face
    ///
    /// If there multiple running timers, show the next one
    fn advance_all_timers_idx(&mut self) {
        let len = self.timers.len();

        self.all_timers_idx = self
            .all_timers_idx
            .map(|current_idx| {
                (1..=len)
                    .map(|offset| (current_idx as usize + offset) % len)
                    .find(|idx| self.timers[*idx].state.is_started())
                    .map(|n| n as u8)
            })
            .unwrap_or_else(|| self.nearest_timer().map(|t| t.idx));
    }

    /// Return the next timer that will go off
    fn nearest_timer(&self) -> Option<&Timer> {
        self.timers
            .iter()
            .filter(|t| t.state.is_started())
            .min_by(|a, b| a.seconds_remaining().cmp(&b.seconds_remaining()))
    }

    fn draw_all_timers_face(&mut self) {
        info!("Drawing all timers face");
        let Some(displayed_timer) = self.all_timers_idx.map(|idx| &self.timers[idx as usize])
        // .as_ref()
        // .or_else(|| self.nearest_timer())
        else {
            // We don't want to be on the AT face if there are no running timers
            // TODO: This shouldn't be in the draw function
            info!("AT timer has no displayed timer anymore");
            self.advance_state();
            self.draw();
            return;
        };

        self.display_indicator_state.signal.set(false);
        unsafe {
            watch_display_string(cstr!("AT        ").as_ptr().cast_mut(), 0);
        }
        watch_display_u8(displayed_timer.idx + 1, false, 2);

        match &displayed_timer.state {
            TimerState::Ready => {
                self.display_indicator_state.tick_frequency.set(1);
                unsafe {
                    watch_clear_colon();
                }

                if self.blink_toggle {
                    unsafe {
                        watch_display_string(cstr!("      ").as_ptr().cast_mut(), 4);
                    }
                } else {
                    unsafe {
                        watch_display_string(cstr!("BEEP  ").as_ptr().cast_mut(), 4);
                    }
                }
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
                    // TODO: Do we need this line if we're immediately clearing it?
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

    fn draw_timer_face(&mut self, timer_n: usize) {
        // info!("Drawing timer face for {timer_n}");
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
                info!("Timer is ready. Wait time: {time_to_wait:?}");

                timer.state = TimerState::Started {
                    time_remaining: time_to_wait.clone(),
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

    fn beep_timer(&mut self, idx: u8) {
        let timer = &mut self.timers[idx as usize];
        info!("Beepiing {idx} times");

        // Reset timer
        timer.state = TimerState::Ready;
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
            timers: array::from_fn(Timer::new),
            num_running_timers: 0,
            timer_presets: DEFAULT_TIMER_PRESETS.clone(),
            display_indicator_state: DisplayIndicatorState::new(),
            all_timers_idx: None,
            blink_toggle: false,
            next_alarm: None,
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

                match self.next_alarm {
                    Some((0, idx)) => {
                        // An alarm is going off
                        self.beep_timer(idx);
                        self.refresh_running_status();
                    }
                    Some((n, idx)) => {
                        self.next_alarm = Some((n.saturating_sub(1), idx));
                    }
                    None => {}
                };

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
                        self.advance_all_timers_idx();
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
                        self.refresh_running_status();
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
            EventType::BackgroundTask => {
                info!("Got background task wakeup");
                unsafe { watch_buzzer_play_sequence(BEEPS.as_ptr() as *mut i8, Some(callback)) }
            }
            EventType::AlarmButtonDown
            | EventType::AlarmLongUp
            | EventType::LightLongUp
            | EventType::LowEnergyUpdate
            | EventType::ModeButtonDown
            | EventType::ModeButtonUp
            | EventType::ModeLongPress
            | EventType::ModeLongUp
            | EventType::None
            | EventType::Timeout
            | EventType::Other(_) => unsafe {
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

    fn face_post_initial_setup(&'static self) {
        // We need a static ref of this context for waking

        // Safety: we can assume that SensorWatch won't free the context, so this pointer is valid for 'static
        // Also, this will only be run once
        unsafe {
            STATE_REF.replace(core::mem::transmute::<&'static Self, *mut Self>(self));
        }
    }
}
