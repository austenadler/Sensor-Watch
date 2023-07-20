#define __SAML22J18A__
#define DONT_USE_CMSIS_INIT
#include "../movement/movement.h"

// #include "../watch-library/shared/watch/watch.h"


// #include <stdbool.h>
// #include <stdint.h>

// typedef union {
//     struct {
//         bool button_should_sound : 1;       // if true, pressing a button emits a sound.
//         uint8_t to_interval : 2;            // an inactivity interval for asking the active face to resign.
//         bool to_always : 1;                 // if true, always time out from the active face to face 0. otherwise only faces that time out will resign (the default).
//         uint8_t le_interval : 3;            // 0 to disable low energy mode, or an inactivity interval for going into low energy mode.
//         uint8_t led_duration : 2;           // how many seconds to shine the LED for (x2), or 0 to disable it.
//         uint8_t led_red_color : 4;          // for general purpose illumination, the red LED value (0-15)
//         uint8_t led_green_color : 4;        // for general purpose illumination, the green LED value (0-15)
//         uint8_t time_zone : 6;              // an integer representing an index in the time zone table.

//         // while Movement itself doesn't implement a clock or display units, it may make sense to include some
//         // global settings for watch faces to check. The 12/24 hour preference could inform a clock or a
//         // time-oriented complication like a sunrise/sunset timer, and a simple locale preference could tell an
//         // altimeter to display feet or meters as easily as it tells a thermometer to display degrees in F or C.
//         bool clock_mode_24h : 1;            // indicates whether clock should use 12 or 24 hour mode.
//         bool use_imperial_units : 1;        // indicates whether to use metric units (the default) or imperial.
//         bool alarm_enabled : 1;             // indicates whether there is at least one alarm enabled.
//         uint8_t reserved : 6;               // room for more preferences if needed.
//     } bit;
//     uint32_t reg;
// } movement_settings_t;

// typedef enum {
//     EVENT_NONE = 0,             // There is no event to report.
//     EVENT_ACTIVATE,             // Your watch face is entering the foreground.
//     EVENT_TICK,                 // Most common event type. Your watch face is being called from the tick callback.
//     EVENT_LOW_ENERGY_UPDATE,    // If the watch is in low energy mode and you are in the foreground, you will get a chance to update the display once per minute.
//     EVENT_BACKGROUND_TASK,      // Your watch face is being invoked to perform a background task. Don't update the display here; you may not be in the foreground.
//     EVENT_TIMEOUT,              // Your watch face has been inactive for a while. You may want to resign, depending on your watch face's intended use case.
//     EVENT_LIGHT_BUTTON_DOWN,    // The light button has been pressed, but not yet released.
//     EVENT_LIGHT_BUTTON_UP,      // The light button was pressed for less than half a second, and released.
//     EVENT_LIGHT_LONG_PRESS,     // The light button was held for over half a second, but not yet released.
//     EVENT_LIGHT_LONG_UP,        // The light button was held for over half a second, and released.
//     EVENT_MODE_BUTTON_DOWN,     // The mode button has been pressed, but not yet released.
//     EVENT_MODE_BUTTON_UP,       // The mode button was pressed for less than half a second, and released.
//     EVENT_MODE_LONG_PRESS,      // The mode button was held for over half a second, but not yet released.
//     EVENT_MODE_LONG_UP,         // The mode button was held for over half a second, and released. NOTE: your watch face will resign immediately after receiving this event.
//     EVENT_ALARM_BUTTON_DOWN,    // The alarm button has been pressed, but not yet released.
//     EVENT_ALARM_BUTTON_UP,      // The alarm button was pressed for less than half a second, and released.
//     EVENT_ALARM_LONG_PRESS,     // The alarm button was held for over half a second, but not yet released.
//     EVENT_ALARM_LONG_UP,        // The alarm button was held for over half a second, and released.
// } movement_event_type_t;

// typedef struct {
//     uint8_t event_type;
//     uint8_t subsecond;
// } movement_event_t;
