use core::fmt::Debug;

use crate::{watch_date_time, watch_rtc_get_date_time, watch_utility_date_time_to_unix_time};
// use crate::watch_utility_date_time_to_unix_time;

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

    pub fn timestamp_utc(&self) -> cty::uint32_t {
        unsafe { watch_utility_date_time_to_unix_time(self.0, 0) }
    }

    // pub fn as_timestamp(&self, settings: *const c_void) -> uint32_t {
    //     unsafe { watch_utility_date_time_to_unix_time(self.0, _get_tz_offset(settings)) }
    // }

    // where _get_tz_offset is:
    // movement_timezone_offsets[settings->bit.time_zone] * 60
}
