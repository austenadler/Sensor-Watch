use cty::uint8_t;
use sensor_watch_sys::{movement_settings_t__bindgen_ty_1, MovementEvent};

#[macro_export]
macro_rules! expose_face {
    ($implementor:ident, $face_name:ident) => {
        paste::paste! {
            use sensor_watch_sys::movement_default_loop_handler;
            use sensor_watch_sys::movement_event_t;
            use sensor_watch_sys::movement_event_type_t;
            use sensor_watch_sys::movement_settings_t;
            use sensor_watch_sys::watch_display_string;
            use cty::{c_void, uint32_t, uint8_t};
            use core::mem;

            #[no_mangle]
            pub extern "C" fn [<$face_name _face_setup>](
                settings: *mut movement_settings_t,
                watch_face_index: uint8_t,
                context_ptr: *mut *mut c_void,
            ) {
                // info!("Called: kitchen_timer_face_setup");
                let settings = unsafe { settings.as_mut().unwrap().bit };

                if unsafe { context_ptr.as_mut().unwrap() }.is_null() {
                    let context = unsafe {
                        *context_ptr = sensor_watch_sys::malloc(mem::size_of::<$implementor>()) as *mut c_void;
                        let context = (*context_ptr as *mut $implementor).as_mut().unwrap();
                        *context = <$implementor as WatchFace>::face_initial_setup(&settings, watch_face_index);
                            info!("Done setting up context");
                        context
                    };
                } else {
                        let mut context = unsafe{(*context_ptr as *mut $implementor).as_mut().unwrap()};
                    // let mut context = unsafe { mem::transmute::<_, &mut $implementor>(context) };
                    context.face_setup(&settings, watch_face_index)
                }
            }
            #[no_mangle]
            pub extern "C" fn [<$face_name _face_activate>](
                settings: *mut movement_settings_t,
                context: *mut c_void,
            ) {
                info!("Called: kitchen_timer_face_activate ({context:?})");
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let mut context = unsafe { mem::transmute::<_, &mut $implementor>(context) };

                context.face_activate(&settings)
            }
            #[no_mangle]
            pub extern "C" fn [<$face_name _face_loop>](
                event: movement_event_t,
                settings: *mut movement_settings_t,
                context: *mut c_void,
            ) -> bool {
                let event = MovementEvent::from(event);
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let mut context = unsafe { mem::transmute::<_, &mut $implementor>(context) };

                context.face_loop(event, &settings)
            }
            #[no_mangle]
            pub extern "C" fn [<$face_name _face_resign>](
                settings: *mut movement_settings_t,
                context: *mut c_void,
            ) {
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let mut context = unsafe { mem::transmute::<_, &mut $implementor>(context) };

                context.face_resign(&settings)
            }
            #[no_mangle]
            pub extern "C" fn [<$face_name _face_wants_background_task>](
                settings: *mut movement_settings_t,
                context: *mut c_void,
            ) -> bool {
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let mut context = unsafe { mem::transmute::<_, &mut $implementor>(context) };

                context.face_wants_background_task(&settings)
            }
        }
    };
}

pub trait WatchFace {
    fn face_initial_setup(
        settings: &movement_settings_t__bindgen_ty_1,
        watch_face_index: uint8_t,
        // context_ptr: *mut *mut c_void,
    ) -> Self;
    fn face_setup(
        &mut self,
        settings: &movement_settings_t__bindgen_ty_1,
        watch_face_index: uint8_t,
        // context_ptr: *mut *mut c_void,
        // settings: *mut movement_settings_t,
        // watch_face_index: uint8_t,
        // context_ptr: *mut *mut c_void,
    ) {
    }
    fn face_activate(
        &mut self,
        settings: &movement_settings_t__bindgen_ty_1,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    );
    fn face_loop(
        &mut self,
        event: MovementEvent,
        settings: &movement_settings_t__bindgen_ty_1,
        // event: movement_event_t,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    ) -> bool;
    fn face_resign(
        &mut self,
        settings: &movement_settings_t__bindgen_ty_1,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    );
    fn face_wants_background_task(
        &mut self,
        settings: &movement_settings_t__bindgen_ty_1,
        // settings: *mut movement_settings_t,
        // context: *mut c_void,
    ) -> bool {
        false
    }
}
