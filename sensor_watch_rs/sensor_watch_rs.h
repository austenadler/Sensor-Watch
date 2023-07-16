#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include "watch.h"


typedef struct MovementEvent MovementEvent;

typedef struct MovementSettings MovementSettings;

void kitchen_timer_face_activate(struct MovementSettings *settings, void **context);

bool kitchen_timer_face_loop(struct MovementEvent *event,
                             struct MovementSettings *settings,
                             void **context);

void kitchen_timer_face_resign(struct MovementSettings *settings, void **context);

void kitchen_timer_face_setup(struct MovementSettings *settings,
                              uint8_t watch_face_index,
                              void **context_ptr);

void set_display_str(void);
