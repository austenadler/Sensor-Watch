#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct MovementEvent;

struct MovementSettings;

extern "C" {

void set_display_str();

extern void watch_display_string(const char *string, uint8_t position);

void kitchen_timer_face_setup(MovementSettings *settings,
                              uint8_t watch_face_index,
                              void **context_ptr);

void kitchen_timer_face_activate(MovementSettings *settings, void **context);

bool kitchen_timer_face_loop(MovementEvent *event, MovementSettings *settings, void **context);

void kitchen_timer_face_resign(MovementSettings *settings, void **context);

} // extern "C"
