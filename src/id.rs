use emscripten_val_sys::bind::*;
use std::ffi::CString;
use std::sync::Once;

use crate::utils::get_type_id;

pub struct GenericWireType(pub f64);

pub trait JsType {
    fn id() -> crate::TYPEID;
    fn signature() -> char {
        'p'
    }
    fn from_generic_wire_type(v: GenericWireType) -> Self;
}

impl JsType for bool {
    fn id() -> crate::TYPEID {
        static INIT: Once = Once::new();
        let type_id = get_type_id::<bool>();

        unsafe {
            INIT.call_once(|| {
                let name = CString::new("rust_bool").unwrap();
                _embind_register_bool(type_id, name.as_ptr(), true, false);
            });
            type_id
        }
    }

    fn signature() -> char {
        'i'
    }

    fn from_generic_wire_type(v: GenericWireType) -> Self {
        v.0 != 0f64
    }
}

macro_rules! register_rust_int {
    ($t:ty, $name:expr) => {
        impl JsType for $t {
            fn id() -> crate::TYPEID {
                static INIT: Once = Once::new();
                let type_id = get_type_id::<$t>();

                unsafe {
                    INIT.call_once(|| {
                        let name_cstr = CString::new($name).unwrap();
                        _embind_register_integer(
                            type_id,
                            name_cstr.as_ptr(),
                            std::mem::size_of::<$t>(),
                            <$t>::MIN as _,
                            <$t>::MAX as _,
                        );
                    });
                    type_id
                }
            }

            fn signature() -> char {
                'i'
            }

            fn from_generic_wire_type(v: GenericWireType) -> Self {
                v.0 as _
            }
        }
    };
}

macro_rules! register_rust_float {
    ($t:ty, $name:expr) => {
        impl JsType for $t {
            fn id() -> crate::TYPEID {
                static INIT: Once = Once::new();
                let type_id = get_type_id::<$t>();

                unsafe {
                    INIT.call_once(|| {
                        let name_cstr = CString::new($name).unwrap();
                        _embind_register_float(
                            type_id,
                            name_cstr.as_ptr(),
                            std::mem::size_of::<$t>(),
                        );
                    });
                    type_id
                }
            }

            fn signature() -> char {
                'd'
            }

            fn from_generic_wire_type(v: GenericWireType) -> Self {
                v.0 as _
            }
        }
    };
}

register_rust_int!(u8, "rust_u8");
register_rust_int!(u16, "rust_u16");
register_rust_int!(u32, "rust_u32");
register_rust_int!(i8, "rust_i8");
register_rust_int!(i16, "rust_i16");
register_rust_int!(i32, "rust_i32");
register_rust_int!(usize, "rust_usize");
register_rust_int!(isize, "rust_isize");
register_rust_float!(f32, "rust_f32");
register_rust_float!(f64, "rust_f64");
