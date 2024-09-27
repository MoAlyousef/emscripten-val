use std::ffi::CString;
use std::os::raw::*;

pub mod sys {
    pub use emscripten_val_sys::sys::*;
}

use sys::*;

fn type_name_of<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn is_pointer<T>(t: &T) -> bool {
    type_name_of(t).starts_with("*const") || type_name_of(t).starts_with("*mut")
}

fn is_integer<T>(t: &T) -> bool {
    matches!(
        type_name_of(t),
        "char" | "i32" | "i64" | "u32" | "u64" | "i16" | "u16" | "i8" | "u8" | "isize" | "usize"
    )
}

fn is_float<T>(t: &T) -> bool {
    matches!(type_name_of(t), "f32" | "f64")
}

fn is_void<T>(t: &T) -> bool {
    type_name_of(t) == "()"
}

pub fn get_argument_type<T>(arg: &T) -> TYPEID {
    unsafe {
        if is_integer(arg) {
            IntType
        } else if is_pointer(arg) {
            PointerType
        } else if is_float(arg) {
            FloatType
        } else if is_void(arg) {
            VoidType
        } else {
            if std::any::type_name::<T>() == std::any::type_name::<Val>() {
                EmvalType
            } else {
                PointerType
            }
        }
    }
}

#[macro_export]
macro_rules! process_arg_types {
    () => {
        vec![]
    };
    ($x:expr, $($rest:tt)*) => {{
        let mut v = vec![get_argument_type(&$x)];
        v.extend_from_slice(&process_arg_types!($($rest)*));
        v
    }};
}

#[macro_export]
macro_rules! process_arguments {
    () => {
        Vec::<*const std::os::raw::c_void>::new()
    };
    ($x:expr, $($rest:tt)*) => {{
        let mut v: Vec<*const std::os::raw::c_void> = vec![std::mem::transmute($x)];
        v.extend_from_slice(&process_arguments!($($rest)*));
        v
    }};
}

#[macro_export]
macro_rules! emval_call_method {
    ($val:tt, $f:literal, $ret:tt, $($rest:tt)*) => {{
        unsafe {
            let f = std::ffi::CString::new($f).unwrap();
            let args = $crate::process_arg_types!($ret, $($rest)*);
            let args_ptr = args.as_ptr();
            let len = args.len() as u32;
            std::mem::forget(args);
            let caller = $crate::sys::_emval_get_method_caller(len, args_ptr as _, 0);
            let argv: Vec<*const std::os::raw::c_void> = $crate::process_arguments!($($rest)*);
            $crate::sys::_emval_call_method(caller, $val.as_handle(), f.as_ptr() as _, std::ptr::null_mut(), argv.as_ptr() as _);
        }
    }}
}

#[macro_export]
macro_rules! gen_args {
    ($($rest:tt)*) => {{
        let mut args = $crate::process_arg_types!($($rest)*);
        let argv: Vec<*const std::os::raw::c_void> = $crate::process_arguments!($($rest)*);
        args.insert(0, $crate::sys::EmvalType);
        (args, argv)
    }}
}

#[repr(C)]
pub struct Val {
    handle: EM_VAL,
}

impl Val {
    pub fn global(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        Self {
            handle: unsafe { _emval_get_global(name.as_ptr()) },
        }
    }

    pub fn take_ownership(v: EM_VAL) -> Self {
        Self {
            handle: v,
        }
    }

    pub fn undefined() -> Self {
        Self {
            handle: _EMVAL_UNDEFINED as EM_VAL,
        }
    }

    pub fn array() -> Self {
        Self {
            handle: unsafe { _emval_new_array() },
        }
    }

    pub fn from_str(s: &str) -> Self {
        let s = CString::new(s).unwrap();
        Self {
            handle: unsafe { _emval_new_u8string(s.as_ptr() as _) }
        }
    }

    pub fn from_numeric_array<T: Clone + Into<f64>>(arr: &[T]) -> Self {
        let v = Val::array();
        for elem in arr {
            let x: f64 = elem.clone().into();
            let x = x as f32;
            unsafe { v.call("push", gen_args![x,]); }
        }
        v
    }

    pub fn from_val_array(arr: &[Val]) -> Self {
        let v = Val::array();
        for elem in arr {
            unsafe { v.call("push", gen_args![elem,]); }
        }
        v
    }

    pub fn as_handle(&self) -> EM_VAL {
        self.handle
    }

    pub unsafe fn call(&self, f: &str, args: (Vec<TYPEID>, Vec<*const c_void>)) -> Val {
        let f = CString::new(f).unwrap();
        let caller = _emval_get_method_caller(args.0.len() as u32, args.0.as_ptr() as _, 0);
        let ret = _emval_call_method(
            caller,
            self.handle,
            f.as_ptr() as _,
            std::ptr::null_mut(),
            args.1.as_ptr() as _,
        );
        let ret = ret as u32 as EM_VAL;
        Val::take_ownership(ret)
    }

    pub fn at(&self, idx: usize) -> Val {
        Val {
            handle: unsafe { _emval_get_property(self.handle, Val::from_u32(idx as _).handle) },
        }
    }

    pub fn from_i32(i: i32) -> Self {
        Self {
            handle: unsafe { _emval_take_value(IntType, [i as *const c_void].as_ptr() as _) },
        }
    }

    pub fn from_u32(i: u32) -> Self {
        Self {
            handle: unsafe { _emval_take_value(IntType, [i as *const c_void].as_ptr() as _) },
        }
    }
}
