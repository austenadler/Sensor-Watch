use proc_macro::{self, TokenStream};
use proc_macro2::{Ident, Span, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Meta, MetaList};

#[proc_macro_derive(WatchFace, attributes(watch_face))]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);
    let watch_face_name = get_watch_face(&attrs);

    let ident_face_setup = Ident::new(&format!("{watch_face_name}_face_setup"), Span::call_site());
    let ident_face_activate = Ident::new(
        &format!("{watch_face_name}_face_activate"),
        Span::call_site(),
    );
    let ident_face_loop = Ident::new(&format!("{watch_face_name}_face_loop"), Span::call_site());
    let ident_face_resign =
        Ident::new(&format!("{watch_face_name}_face_resign"), Span::call_site());
    let ident_face_wants_background_task = Ident::new(
        &format!("{watch_face_name}_face_wants_background_task"),
        Span::call_site(),
    );

    quote! {
        #[no_mangle]
        pub extern "C" fn #ident_face_setup(
            settings: *mut ::sensor_watch_rs::sys::movement_settings_t,
            watch_face_index: ::cty::uint8_t,
            context_ptr: *mut *mut ::cty::c_void,
        ) {
            let settings = unsafe { settings.as_mut().unwrap().bit };

            if unsafe { context_ptr.as_mut().unwrap() }.is_null() {
                let context: &'static #ident = unsafe {
                    *context_ptr = ::sensor_watch_rs::sys::malloc(::core::mem::size_of::<::core::mem::MaybeUninit<#ident>>()) as *mut ::core::ffi::c_void;
                    let context = (*context_ptr as *mut ::core::mem::MaybeUninit<#ident>).as_mut().unwrap();
                    (*context).write(<#ident as WatchFace>::face_initial_setup(settings, watch_face_index));
                    context.assume_init_mut()
                };

                context.face_post_initial_setup();
            } else {
                let context = unsafe{(*context_ptr as *mut #ident).as_mut().unwrap()};
                context.face_setup(settings, watch_face_index)
            }
        }

        #[no_mangle]
        pub extern "C" fn #ident_face_activate(
            settings: *mut ::sensor_watch_rs::sys::movement_settings_t,
            context: *mut ::cty::c_void,
        ) {
            info!("Called: kitchen_timer_face_activate ({context:?})");
            let settings = unsafe { settings.as_mut().unwrap().bit };
            let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

            context.face_activate(settings)
        }

        #[no_mangle]
        pub extern "C" fn #ident_face_loop(
            event: ::sensor_watch_rs::sys::movement_event_t,
            settings: *mut ::sensor_watch_rs::sys::movement_settings_t,
            context: *mut ::cty::c_void,
        ) -> bool {
            let event = MovementEvent::from(event);
            let settings = unsafe { settings.as_mut().unwrap().bit };
            let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

            context.face_loop(event, settings)
        }

        #[no_mangle]
        pub extern "C" fn #ident_face_resign(
            settings: *mut ::sensor_watch_rs::sys::movement_settings_t,
            context: *mut ::cty::c_void,
        ) {
            let settings = unsafe { settings.as_mut().unwrap().bit };
            let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

            context.face_resign(settings)
        }

        #[no_mangle]
        pub extern "C" fn #ident_face_wants_background_task(
            settings: *mut ::sensor_watch_rs::sys::movement_settings_t,
            context: *mut ::cty::c_void,
        ) -> bool {
            let settings = unsafe { settings.as_mut().unwrap().bit };
            let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

            context.face_wants_background_task(settings)
        }
    }.into()
}

fn get_watch_face(attrs: &[Attribute]) -> String {
    let panic_message = "#[watch_face(...)] attribute is required";

    match &attrs
        .iter()
        .find(|w| {
            if w.meta.path().get_ident().map(Ident::to_string).as_deref() == Some("watch_face") {
                true
            } else {
                panic!("Unknown attribute. {panic_message}");
            }
        })
        .unwrap_or_else(|| panic!("{panic_message}"))
        .meta
    {
        Meta::List(MetaList { tokens, .. }) => match tokens.clone().into_iter().next() {
            Some(TokenTree::Ident(l)) => l.to_string(),
            Some(t) => {
                panic!("Ident required; {t:?} given");
            }
            None => {
                panic!("watch_face requires one argument");
            }
        },
        i => {
            panic!("Unknown attribute type {i:?} {panic_message}");
        }
    }
}
