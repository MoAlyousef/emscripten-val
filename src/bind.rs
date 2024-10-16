use emscripten_val_sys::bind::*;
use std::any::TypeId;
use std::ffi::CString;
use std::os::raw::*;

use crate::utils::get_type_id;
pub use emscripten_val_sys::bind::_embind_register_class_property;

pub fn register_class<T: 'static>(name: &str) {
    extern "C" fn get_actual_type<T: 'static>(_arg: *const c_void) -> crate::TYPEID {
        get_type_id::<T>()
    }
    extern "C" fn noop() {}
    extern "C" fn destructor<T: 'static>(arg: *mut c_void) {
        let _ = unsafe { Box::from_raw(arg as *mut T) };
    }
    let typ = get_type_id::<T>();
    let ptr_typ = get_type_id::<*mut T>();
    let const_ptr_typ = get_type_id::<*const T>();
    let name = CString::new(name).unwrap();
    unsafe {
        _embind_register_class(
            typ,
            ptr_typ,
            const_ptr_typ,
            std::ptr::null(),
            "ii\0".as_ptr() as _,
            get_actual_type::<T> as _,
            "v\0".as_ptr() as _,
            noop as _,
            "v\0".as_ptr() as _,
            noop as _,
            name.as_ptr() as _,
            "vi\0".as_ptr() as _,
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
            extern "C" fn getter(_ctx: i32, ptr: $cls) -> $membertype {
                unsafe { ptr.$member }
            }

            extern "C" fn setter(_ctx: i32, mut ptr: $cls, value: $membertype) {
                unsafe {
                    ptr.$member = value;
                    std::mem::forget(ptr);
                }
            }
            let cname = std::ffi::CString::new($name).unwrap();
            _embind_register_class_property(
                $crate::utils::get_type_id::<$cls>(),
                cname.as_ptr() as _,
                $crate::utils::get_type_id::<$membertype>(),
                "iii\0".as_ptr() as _,
                getter as _,
                std::ptr::null_mut(),
                $crate::utils::get_type_id::<$membertype>(),
                "viii\0".as_ptr() as _,
                setter as _,
                std::ptr::null_mut(),
            );
        };
        f();
    }};
}
