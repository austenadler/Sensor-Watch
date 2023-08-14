use core::{ffi::CStr, fmt::Debug, ops::Add};

use cty::uint32_t;

use crate::{
    movement_schedule_background_task_for_face, watch_date_time, watch_display_string,
    watch_rtc_get_date_time, watch_set_colon, watch_utility_date_time_from_unix_time,
    watch_utility_date_time_to_unix_time, watch_utility_offset_timestamp,
    watch_utility_seconds_to_duration, write_u8_chars, info,
};
// use crate::watch_utility_date_time_to_unix_time;

// TODO: This probably is a u32, not a watch_date_time
pub struct WatchDateTime(watch_date_time);

impl Debug for WatchDateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // We can assume this will be safe
        let value = unsafe { self.0.unit };

        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            value.year(),
            value.month(),
            value.day(),
            value.hour(),
            value.minute(),
            value.second(),
        )
    }
}

impl WatchDateTime {
    pub fn now() -> Self {
        Self(unsafe { watch_rtc_get_date_time() })
    }

    pub fn from_utc_secs(seconds: uint32_t) -> Self {
        Self(unsafe { watch_utility_date_time_from_unix_time(seconds, 0) })
    }

    pub fn timestamp_utc(&self) -> cty::uint32_t {
        unsafe { watch_utility_date_time_to_unix_time(self.0, 0) }
    }

    pub fn schedule_background_task_for_face(&self, watch_face_index: u8) {
        info!("Scheduling backgorund task for face {watch_face_index} for time {self:?}");
        unsafe {
            movement_schedule_background_task_for_face(watch_face_index, self.0);
        }
    }
}

impl Add<TimeEntry> for WatchDateTime {
    type Output = Self;

    fn add(self, rhs: TimeEntry) -> Self::Output {
        // TODO: This might not be super efficient
                Self::from_utc_secs(self.timestamp_utc() + rhs.as_seconds())
    }
}

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
