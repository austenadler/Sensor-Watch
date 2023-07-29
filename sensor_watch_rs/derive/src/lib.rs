use proc_macro::{self, TokenStream};
use proc_macro2::Ident;
use proc_macro2::Span;
use syn::Attribute;
use quote::quote;
use proc_macro2::TokenTree;
use syn::Lit;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed, MetaNameValue, Meta, MetaList};

#[proc_macro_derive(WatchFace, attributes(watch_face))]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, attrs, .. } = parse_macro_input!(input);
    let watch_face_name = get_watch_face(&attrs);

    let ident_face_setup = Ident::new(&format!("{watch_face_name}_face_setup"), Span::call_site());
    let ident_face_activate = Ident::new(&format!("{watch_face_name}_face_activate"), Span::call_site());
    let ident_face_loop = Ident::new(&format!("{watch_face_name}_face_loop"), Span::call_site());
    let ident_face_resign = Ident::new(&format!("{watch_face_name}_face_resign"), Span::call_site());
    let ident_face_wants_background_task = Ident::new(
        &format!("{watch_face_name}_face_wants_background_task"),
        Span::call_site(),
    );

    // let description = match data {
    // syn::Data::Struct(s) => match s.fields {
    //     syn::Fields::Named(FieldsNamed { named, .. }) => {
    //     let idents = named.iter().map(|f| &f.ident);
    //     format!(
    //         "a struct with these named fields: {}",
    //         quote! {#(#idents), *}
    //     )
    //     }
    //     syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
    //     let num_fields = unnamed.iter().count();
    //     format!("a struct with {} unnamed fields", num_fields)
    //     }
    //     syn::Fields::Unit => format!("a unit struct"),
    // },
    // syn::Data::Enum(DataEnum { variants, .. }) => {
    //     let vs = variants.iter().map(|v| &v.ident);
    //     format!("an enum with these variants: {}", quote! {#(#vs),*})
    // }
    // syn::Data::Union(DataUnion {
    //     fields: FieldsNamed { named, .. },
    //     ..
    // }) => {
    //     let idents = named.iter().map(|f| &f.ident);
    //     format!("a union with these named fields: {}", quote! {#(#idents),*})
    // }
    // };

    // let output = quote! {
    // impl #ident {
    //     fn describe() {
    //     println!("{} is {}.", stringify!(#ident), #description);
    //     }
    // }
    // };

    // output.into()
    // let mut ret = proc_macro2::TokenStream::from(input.clone());
    // let mut ret = proc_macro2::TokenStream::new();

    // ret.extend();
    // ret.into()
        quote! {
            #[no_mangle]
            pub extern "C" fn #ident_face_setup(
                settings: *mut ::sensor_watch_sys::movement_settings_t,
                watch_face_index: ::cty::uint8_t,
                context_ptr: *mut *mut ::cty::c_void,
            ) {
                let settings = unsafe { settings.as_mut().unwrap().bit };

                if unsafe { context_ptr.as_mut().unwrap() }.is_null() {
                    let _context = unsafe {
                        *context_ptr = ::sensor_watch_sys::malloc(::core::mem::size_of::<::core::mem::MaybeUninit<#ident>>()) as *mut ::core::ffi::c_void;
                        let context = (*context_ptr as *mut ::core::mem::MaybeUninit<#ident>).as_mut().unwrap();
                        (*context).write(<#ident as WatchFace>::face_initial_setup(settings, watch_face_index));
                        context.assume_init_mut()
                    };
                } else {
                    let context = unsafe{(*context_ptr as *mut #ident).as_mut().unwrap()};
                    context.face_setup(settings, watch_face_index)
                }
            }
            #[no_mangle]
            pub extern "C" fn #ident_face_activate(
                settings: *mut ::sensor_watch_sys::movement_settings_t,
                context: *mut ::cty::c_void,
            ) {
                info!("Called: kitchen_timer_face_activate ({context:?})");
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

                context.face_activate(settings)
            }
            #[no_mangle]
            pub extern "C" fn #ident_face_loop(
                event: ::sensor_watch_sys::movement_event_t,
                settings: *mut ::sensor_watch_sys::movement_settings_t,
                context: *mut ::cty::c_void,
            ) -> bool {
                let event = MovementEvent::from(event);
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

                context.face_loop(event, settings)
            }
            #[no_mangle]
            pub extern "C" fn #ident_face_resign(
                settings: *mut ::sensor_watch_sys::movement_settings_t,
                context: *mut ::cty::c_void,
            ) {
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

                context.face_resign(settings)
            }
            #[no_mangle]
            pub extern "C" fn #ident_face_wants_background_task(
                settings: *mut ::sensor_watch_sys::movement_settings_t,
                context: *mut ::cty::c_void,
            ) -> bool {
                let settings = unsafe { settings.as_mut().unwrap().bit };
                let context = unsafe { ::core::mem::transmute::<_, &mut #ident>(context) };

                context.face_wants_background_task(settings)
            }
    }.into()
}

fn get_watch_face(attrs: &[Attribute]) -> String {
    let panic_message = "#[\"watch_face\"] attribute is required";

    match &attrs.iter().find(
        |w| if w.meta.path().get_ident().map(Ident::to_string).as_deref() == Some("watch_face") {
                true
            } else {
                panic!("Unknown attribute. {panic_message}");
            }
    //         let (path, value) = match attribute.parse_meta().unwrap() {
    // syn::Meta::NameValue(syn::MetaNameValue {
    //     path,
    //     lit: syn::Lit::Str(s),
    //     ..
    // }) => (path, s.value()),
    // _ => panic!("malformed attribute syntax"),
    ).unwrap_or_else(|| panic!("{panic_message}")).meta {
        // Meta::NameValue(MetaNameValue {path, value,..}) => {
        //     panic!("Path: {path:?}, lit: {value:?}")
        // }
        Meta::List(MetaList {tokens, ..}) =>{
            match tokens.clone().into_iter().next() {
                Some(TokenTree::Ident(l)) => {
                    l.to_string()
                }
                Some(t) => {
                    panic!("Ident required; {t:?} given");
                }
                None => {
                    panic!("watch_face requires one argument");
                }
            }
            // let Some(t) = tokens.get(0)

        }
        i => {
            panic!("Unknown attribute type {i:?} {panic_message}");
        }
    }

    // let watch_face_name = &
    // ;

    // panic!("attrs: {attrs:#?} ({watch_face_name:#?})");
}
