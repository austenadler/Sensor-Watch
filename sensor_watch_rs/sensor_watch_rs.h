#ifndef SENSOR_WATCH_RS_H
#define SENSOR_WATCH_RS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include "watch.h"


typedef struct MovementEvent MovementEvent;

typedef struct MovementSettingsInner {
  bool button_should_sound;
  uint8_t to_interval;
  bool to_always;
  uint8_t le_interval;
  uint8_t led_duration;
  uint8_t led_red_color;
  uint8_t led_green_color;
  uint8_t time_zone;
  bool clock_mode_24h;
  bool use_imperial_units;
  bool alarm_enabled;
  uint8_t reserved;
} MovementSettingsInner;

typedef struct MovementSettings {
  uint32_t reg;
  struct MovementSettingsInner bit;
} MovementSettings;

void kitchen_timer_face_activate(struct MovementSettings *settings, void **context);

bool kitchen_timer_face_loop(struct MovementEvent *event,
                             struct MovementSettings *settings,
                             void **context);

void kitchen_timer_face_resign(struct MovementSettings *settings, void **context);

void kitchen_timer_face_setup(struct MovementSettings *settings,
                              uint8_t watch_face_index,
                              void **context_ptr);

void set_display_str(void);

#endif /* SENSOR_WATCH_RS_H */
