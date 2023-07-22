use core::mem;

use cty::{c_void, uint32_t, uint8_t};

use cstr::cstr;
use sensor_watch_sys::movement_default_loop_handler;
use sensor_watch_sys::movement_event_t;
use sensor_watch_sys::movement_event_type_t;
use sensor_watch_sys::movement_settings_t;
use sensor_watch_sys::watch_display_string;
use sensor_watch_sys::MovementEvent;

use sensor_watch_sys::info;

#[derive(Debug)]
#[repr(C)]
struct Context {
    x: u8,
    y: u8,
    watch_face_index: uint8_t,
}

impl Context {
    fn new(watch_face_index: uint8_t) -> Self {
        Self {
            x: 0,
            y: 0,
            watch_face_index,
        }
    }
}

#[no_mangle]
pub extern "C" fn kitchen_timer_face_setup(
    settings: *mut movement_settings_t,
    watch_face_index: uint8_t,
    context_ptr: *mut *mut c_void,
) {
    info!("Called: kitchen_timer_face_setup");
    let settings = unsafe { settings.as_mut().unwrap().bit };

    if unsafe { context_ptr.as_mut().unwrap() }.is_null() {
        let context = unsafe {
            *context_ptr = sensor_watch_sys::malloc(mem::size_of::<Context>()) as *mut c_void;
            let context = (*context_ptr as *mut Context).as_mut().unwrap();
            *context = Context::new(watch_face_index);
            context
        };

        info!("Initialized context as {context:?}");
    }
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_activate(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) {
    info!("Called: kitchen_timer_face_activate ({context:?})");
    let settings = unsafe { settings.as_mut().unwrap().bit };
    let mut context = unsafe { mem::transmute::<_, &mut Context>(context) };
    context.x +=1;
    unsafe {
        watch_display_string(
            cstr!("HI  DANI").as_ptr().cast_mut(),
            0,
        )
    };
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_loop(
    event: movement_event_t,
    settings: *mut movement_settings_t,
    context: *mut c_void,
) -> bool {
    let event = MovementEvent::from(event);
    let mut context = unsafe { mem::transmute::<_, &mut Context>(context) };

    info!("Event: {event:?} ({context:?})");

    match event.event_type {
        _ => unsafe {
            movement_default_loop_handler(event.into(), settings);
        },
    }
    false
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_resign(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) {
    let settings = unsafe { settings.as_mut().unwrap().bit };
    let mut context = unsafe { mem::transmute::<_, &mut Context>(context) };

    info!(
        "Called: kitchen_timer_face_resign ({})",
        settings.time_zone()
    );
}
#[no_mangle]
pub extern "C" fn kitchen_timer_face_wants_background_task(
    settings: *mut movement_settings_t,
    context: *mut c_void,
) -> bool {
    let settings = unsafe { settings.as_mut().unwrap().bit };
    let mut context = unsafe { mem::transmute::<_, &mut Context>(context) };

    info!("Called: kitchen_timer_face_wants_background_task");
    false
}
