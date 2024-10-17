use emscripten_val_sys::bind::*;
use std::any::TypeId;
use std::ffi::CString;
use std::os::raw::*;

use crate::utils::get_type_id;
pub use emscripten_val_sys::bind::_embind_register_class_property;

pub fn register_class<T: 'static>(name: &str) {
    let type_id = get_type_id::<T>();
    let ptr_type_id = get_type_id::<*mut T>();
    let const_ptr_type_id = get_type_id::<*const T>();
    let name_cstr = CString::new(name).unwrap();

    extern "C" fn get_actual_type<T: 'static>(_ptr: *const ()) -> crate::TYPEID {
        get_type_id::<T>()
    }

    extern "C" fn upcast(ptr: *const ()) -> *const () {
        ptr
    }

    extern "C" fn downcast(ptr: *const ()) -> *const () {
        ptr
    }

    extern "C" fn destructor<T>(ptr: *mut T) {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }

    unsafe {
        _embind_register_class(
            type_id,
            ptr_type_id,
            const_ptr_type_id,
            std::ptr::null() as _, // No base class
            "pp\0".as_ptr() as _,
            get_actual_type::<T> as _,
            "pp\0".as_ptr() as _,
            upcast as _,
            "pp\0".as_ptr() as _,
            downcast as _,
            name_cstr.as_ptr(),
            "vp\0".as_ptr() as _,
            destructor::<T> as _,
        );
    }
}

pub fn register_class_default_constructor<T: 'static + Default>() {
    extern "C" fn invoker(f: extern "C" fn() -> *mut c_void) -> *mut c_void {
        f()
    }

    unsafe {
        let arg_types = [TypeId::of::<*mut T>()];
        _embind_register_class_constructor(
            get_type_id::<T>(),
            arg_types.len() as u32,
            arg_types.as_ptr() as _,
            "ii\0".as_ptr() as _,
            invoker as _,
            std::mem::transmute(Box::<T>::default as fn() -> Box<_>),
        )
    }
}

#[macro_export]
macro_rules! register_class_property {
    ($cls:ty, $name:literal, $member:ident, $membertype:ty) => {{
        let f = || unsafe {
            extern "C" fn getter(ptr: *const $cls) -> $membertype {
                unsafe {
                    (*ptr).$member
                }
            }

            extern "C" fn setter(ptr: *mut $cls, value: $membertype) {
                unsafe {
                    (*ptr).$member = value;
                }
            }

            let cname = std::ffi::CString::new($name).unwrap();

            let getter_signature = concat!(
                stringify!(<$membertype>::signature()),
                "p\0"
            );

            let setter_signature = concat!(
                "vp",
                stringify!(<$membertype>::signature()),
                "\0"
            );

            _embind_register_class_property(
                $crate::utils::get_type_id::<$cls>(),
                cname.as_ptr(),
                <$membertype>::id(),
                getter_signature.as_ptr() as _,
                getter as _,
                std::ptr::null_mut(),
                <$membertype>::id(),
                setter_signature.as_ptr() as _,
                setter as _,
                std::ptr::null_mut(),
            );
        };
        f();
    }};
}