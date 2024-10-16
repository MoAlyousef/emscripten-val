use emscripten_val_sys::bind::*;
use emscripten_val_sys::val;
use std::sync::Once;

use crate::utils::get_type_id;

pub struct GenericWireType(pub f64);

pub trait JsType {
    fn id() -> val::TYPEID;
    fn from_generic_wire_type(v: GenericWireType) -> Self;
}

impl JsType for bool {
    fn id() -> val::TYPEID {
        static INIT: Once = Once::new();
        let v = get_type_id::<bool>();
        static BOOL_ID: &str = "rust_bool\0";

        INIT.call_once(|| unsafe {
            _embind_register_bool(v, BOOL_ID.as_ptr() as _, true, false);
        });

        v
    }
    fn from_generic_wire_type(v: GenericWireType) -> Self {
        v.0 != 0f64
    }
}

macro_rules! register_rust_int {
    ($t:ident) => {
        impl JsType for $t {
            fn id() -> val::TYPEID {
                static INIT: Once = Once::new();
                let v = get_type_id::<$t>();
                static INT_ID: &str = concat!("rust_", stringify!($t), "\0");

                INIT.call_once(|| unsafe {
                    _embind_register_integer(
                        v,
                        INT_ID.as_ptr() as _,
                        size_of::<$t>(),
                        $t::MIN as _,
                        $t::MAX as _,
                    );
                });

                v
            }

            fn from_generic_wire_type(v: GenericWireType) -> Self {
                v.0 as _
            }
        }
    };
}

macro_rules! register_rust_float {
    ($t:ident) => {
        impl JsType for $t {
            fn id() -> val::TYPEID {
                static INIT: Once = Once::new();
                let v = get_type_id::<$t>();
                static DOUBLE_ID: &str = concat!("rust_", stringify!($t), "\0");

                INIT.call_once(|| unsafe {
                    _embind_register_float(v, DOUBLE_ID.as_ptr() as _, size_of::<$t>());
                });

                v
            }

            fn from_generic_wire_type(v: GenericWireType) -> Self {
                v.0 as _
            }
        }
    };
}

register_rust_int!(u8);
register_rust_int!(u16);
register_rust_int!(u32);
register_rust_int!(i8);
register_rust_int!(i16);
register_rust_int!(i32);
register_rust_int!(usize);
register_rust_int!(isize);
register_rust_float!(f32);
register_rust_float!(f64);
