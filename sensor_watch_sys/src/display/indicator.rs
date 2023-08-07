use crate::movement_request_tick_frequency;
use crate::watch_clear_indicator;
use crate::watch_set_indicator;
use crate::WatchIndicatorSegment;

/// Helper to not re-clear already cleared indicators
#[derive(Debug)]
pub struct DisplayIndicatorState {
    pub h_24: Idempotent<bool>,
    pub bell: Idempotent<bool>,
    pub lap: Idempotent<bool>,
    pub pm: Idempotent<bool>,
    pub signal: Idempotent<bool>,
    pub tick_frequency: Idempotent<u8>,
}

#[derive(Debug)]
pub struct Idempotent<T: PartialEq> {
    value: T,
    on_change: fn(&T),
}

impl<T: PartialEq> Idempotent<T> {
    pub fn new(value: T, on_change: fn(&T)) -> Idempotent<T> {
        Self { value, on_change }
    }

    pub fn set(&mut self, new_value: T) {
        if new_value != self.value {
            (self.on_change)(&new_value);
        }
        self.value = new_value;
    }
}

impl DisplayIndicatorState {
    pub fn resign(&mut self) {
        self.tick_frequency.set(1);
    }

    pub fn new() -> Self {
        Self {
            h_24: Idempotent::new(false, |new_value| {
                if *new_value {
                    unsafe {
                        watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                } else {
                    unsafe {
                        watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                }
            }),
            bell: Idempotent::new(false, |new_value| {
                if *new_value {
                    unsafe {
                        watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                } else {
                    unsafe {
                        watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                }
            }),
            lap: Idempotent::new(false, |new_value| {
                if *new_value {
                    unsafe {
                        watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                } else {
                    unsafe {
                        watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                }
            }),
            pm: Idempotent::new(false, |new_value| {
                if *new_value {
                    unsafe {
                        watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                } else {
                    unsafe {
                        watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                }
            }),
            signal: Idempotent::new(false, |new_value| {
                if *new_value {
                    unsafe {
                        watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                } else {
                    unsafe {
                        watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                    }
                }
            }),
            tick_frequency: Idempotent::new(1, |new_value| unsafe {
                movement_request_tick_frequency(*new_value);
            }),
        }
    }
}
