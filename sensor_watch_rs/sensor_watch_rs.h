#ifndef SENSOR_WATCH_RS_H
#define SENSOR_WATCH_RS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include "watch.h"


void kitchen_timer_face_activate(movement_settings_t *settings, void *context);

bool kitchen_timer_face_loop(movement_event_t event, movement_settings_t *settings, void *context);

void kitchen_timer_face_resign(movement_settings_t *settings, void *context);

void kitchen_timer_face_setup(movement_settings_t *settings,
                              uint8_t watch_face_index,
                              void **context_ptr);

void set_display_str(void);

#endif /* SENSOR_WATCH_RS_H */
