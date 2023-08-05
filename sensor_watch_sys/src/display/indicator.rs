use crate::watch_clear_indicator;
use crate::watch_set_indicator;
use crate::WatchIndicatorSegment;

/// Helper to not re-clear already cleared indicators
#[derive(Debug, Default)]
pub struct DisplayIndicatorState {
    h_24: bool,
    bell: bool,
    lap: bool,
    pm: bool,
    signal: bool,
}

impl DisplayIndicatorState {
    pub fn set_24h(&mut self, state: bool) {
        if self.h_24 != state {
            self.h_24 = state;

            if state {
                unsafe {
                    watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_24H);
                }
            } else {
                unsafe {
                    watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_24H);
                }
            }
        }
    }
    pub fn set_bell(&mut self, state: bool) {
        if self.bell != state {
            self.bell = state;

            if state {
                unsafe {
                    watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_BELL);
                }
            } else {
                unsafe {
                    watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_BELL);
                }
            }
        }
    }
    pub fn set_lap(&mut self, state: bool) {
        if self.lap != state {
            self.lap = state;

            if state {
                unsafe {
                    watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_LAP);
                }
            } else {
                unsafe {
                    watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_LAP);
                }
            }
        }
    }
    pub fn set_pm(&mut self, state: bool) {
        if self.pm != state {
            self.pm = state;

            if state {
                unsafe {
                    watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                }
            } else {
                unsafe {
                    watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_PM);
                }
            }
        }
    }
    pub fn set_signal(&mut self, state: bool) {
        if self.signal != state {
            self.signal = state;

            if state {
                unsafe {
                    watch_set_indicator(WatchIndicatorSegment::WATCH_INDICATOR_SIGNAL);
                }
            } else {
                unsafe {
                    watch_clear_indicator(WatchIndicatorSegment::WATCH_INDICATOR_SIGNAL);
                }
            }
        }
    }
}
