use cstr::cstr;
use cty::uint8_t;
use sensor_watch_rs::{
    derive::WatchFace,
    display::DisplayIndicatorState,
    face::WatchFace,
    info,
    sys::{
        movement_default_loop_handler, movement_settings_t, movement_settings_t__bindgen_ty_1,
        watch_buzzer_play_sequence, watch_disable_buzzer, watch_display_string,
        watch_enable_buzzer, watch_is_buzzer_or_led_enabled, BuzzerNote,
    },
    time::WatchDateTime,
    EventType, MovementEvent,
};

static HOUR_BEEP: &[i8] = &[
    BuzzerNote::BUZZER_NOTE_C8.0 as i8,
    3,
    BuzzerNote::BUZZER_NOTE_REST.0 as i8,
    3,
    -2,
    1,
    // BuzzerNote::BUZZER_NOTE_C8.0 as i8,
    // 5,
    // BuzzerNote::BUZZER_NOTE_REST.0 as i8,
    // 25,
    0,
];

static THIRTY_MINUTE_BEEP: &[i8] = &[
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
static DEBUG_BEEP: &[i8] = &[
    BuzzerNote::BUZZER_NOTE_B8.0 as i8,
    3,
    BuzzerNote::BUZZER_NOTE_REST.0 as i8,
    3,
    // -2,
    // 2,
    BuzzerNote::BUZZER_NOTE_E8.0 as i8,
    5,
    BuzzerNote::BUZZER_NOTE_REST.0 as i8,
    25,
    0,
];

#[derive(Debug, WatchFace)]
#[watch_face(ships_bell_lite)]
// TODO: Is it unsafe to libc::malloc a non-repr(C) object, even if it's only accessed within rust?
// #[repr(C)]
struct Context {
    active: bool,
    display_indicator_state: DisplayIndicatorState,
    _watch_face_index: uint8_t,
}

impl Context {
    fn draw(&mut self) {
        self.display_indicator_state.bell.set(self.active);
    }

    fn activate(&mut self, active: bool) {
        if self.active != active {
            self.active = active;
            self.draw();
        }
    }
}

impl WatchFace for Context {
    fn face_initial_setup(
        _settings: movement_settings_t__bindgen_ty_1,
        watch_face_index: uint8_t,
    ) -> Self {
        Self {
            active: true,
            display_indicator_state: DisplayIndicatorState::new(),
            _watch_face_index: watch_face_index,
        }
    }

    fn face_activate(&mut self, _settings: movement_settings_t__bindgen_ty_1) {
        info!("In face_activate {self:?}");
        unsafe { watch_display_string(cstr!("SB  ").as_ptr().cast_mut(), 0) };
        self.draw();
    }

    fn face_loop(
        &mut self,
        event: MovementEvent,
        settings: movement_settings_t__bindgen_ty_1,
    ) -> bool {
        match event.event_type {
            EventType::AlarmButtonDown => {
                self.activate(!self.active);
            }
            EventType::LightButtonDown | EventType::BackgroundTask => {
                info!("Light button down");
                let now = WatchDateTime::now();
                let pattern = match now.minute() {
                    0 => HOUR_BEEP,
                    30 => THIRTY_MINUTE_BEEP,
                    _ => DEBUG_BEEP,
                    // _ => return true,
                };

                if unsafe { watch_is_buzzer_or_led_enabled() } {
                    unsafe { watch_buzzer_play_sequence(pattern.as_ptr() as *mut i8, None) }
                } else {
                    unsafe {
                        watch_enable_buzzer();
                        watch_buzzer_play_sequence(pattern.as_ptr() as *mut i8, None);
                        // TODO: In the emulator, this immediately disables the buzzer, so no sound comes out
                        // I do not know if this is true of the real watch
                        // watch_disable_buzzer();
                    }
                }
            }
            _ => unsafe {
                movement_default_loop_handler(event.into(), &mut (settings.into()));
            },
        }

        unsafe {
            movement_default_loop_handler(event.into(), &mut (settings.into()));
        }

        true
    }

    fn face_resign(&mut self, _settings: movement_settings_t__bindgen_ty_1) {
        info!("In face_resign {self:?}");

        core::mem::replace(
            &mut self.display_indicator_state,
            DisplayIndicatorState::new(),
        )
        .resign();
    }

    fn face_wants_background_task(&mut self, _settings: movement_settings_t__bindgen_ty_1) -> bool {
        info!(
            "In face_wants_background_task for ships bell ({})",
            self.active
        );
        if !self.active {
            return false;
        }

        let now = WatchDateTime::now();
        match now.minute() {
            0 | 30 => true,
            _ => false,
        }
    }
}
