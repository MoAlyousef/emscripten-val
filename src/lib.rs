use std::ffi::CString;
use std::os::raw::*;

pub mod sys {
    pub use emscripten_val_sys::sys::*;
}

use sys::*;

pub type EventListenerCb = Option<unsafe extern "C" fn(Val, *mut c_void)>;

fn type_name_of<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn is_pointer<T>(t: &T) -> bool {
    type_name_of(t).starts_with("*const") || type_name_of(t).starts_with("*mut")
}

fn is_integer<T>(t: &T) -> bool {
    matches!(
        type_name_of(t),
        "char" | "i32" | "i64" | "u32" | "u64" | "i16" | "u16" | "i8" | "u8" | "isize" | "usize" | "true" | "false"
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
                dbg!(type_name_of(arg));
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
    ($x:expr) => {{
        vec![get_argument_type(&$x)]
    }};
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
    ($x:expr) => {{
        let mut v: Vec<*const std::os::raw::c_void> = vec![std::mem::transmute($x)];
        v
    }};
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

#[no_mangle]
extern "C" fn rust_caller(val: EM_VAL, data: *mut c_void) {
    let a: *mut Box<dyn FnMut(&Val)> = data as *mut Box<dyn FnMut(&Val)>;
    let mut a = unsafe { Box::from_raw(a) };
    (*a)(&Val::take_ownership(val));
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
        Self { handle: v }
    }

    pub fn from_val(v: Val) -> Self {
        Self { handle: v.handle }
    }

    pub fn undefined() -> Self {
        Self {
            handle: _EMVAL_UNDEFINED as EM_VAL,
        }
    }

    pub fn object() -> Self {
        Self {
            handle: unsafe { _emval_new_object() },
        }
    }

    pub fn null() -> Self {
        Self {
            handle: _EMVAL_NULL as EM_VAL,
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
            handle: unsafe { _emval_new_cstring(s.as_ptr() as _) },
        }
    }

    pub fn module_property(s: &str) -> Self {
        let s = CString::new(s).unwrap();
        Self {
            handle: unsafe { _emval_get_module_property(s.as_ptr() as _) },
        }
    }

    pub fn from_numeric_array<T: Clone + Into<f64>>(arr: &[T]) -> Self {
        let v = Val::array();
        for elem in arr {
            let x: f64 = elem.clone().into();
            let x = x as f32;
            unsafe {
                v.call("push", gen_args![x,]);
            }
        }
        v
    }

    pub fn from_val_array(arr: &[Val]) -> Self {
        let v = Val::array();
        for elem in arr {
            unsafe {
                v.call("push", gen_args![elem,]);
            }
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

    pub fn set(&self, prop: &Val, val: &Val) {
        unsafe { _emval_set_property(self.handle, prop.handle, val.handle) };
    }

    pub fn from_i32(i: i32) -> Self {
        // TODO: check val_ref
        Self {
            handle: unsafe { _emval_take_value(IntType, [i as *const c_void].as_ptr() as _) },
        }
    }

    pub fn from_u32(i: u32) -> Self {
        Self {
            handle: unsafe { _emval_take_value(IntType, [i as *const c_void].as_ptr() as _) },
        }
    }

    pub fn from_f32(i: f32) -> Self {
        let i: *const c_void = unsafe { std::mem::transmute(i) };
        Self {
            handle: unsafe { _emval_take_value(FloatType, [i].as_ptr() as _) },
        }
    }

    pub fn from_f64(i: f64) -> Self {
        let i: *const c_void = unsafe { std::mem::transmute(i as f32) };
        Self {
            handle: unsafe { _emval_take_value(FloatType, [i].as_ptr() as _) },
        }
    }

    pub fn from_bool(i: bool) -> Self {
        Self {
            handle: if i { _EMVAL_TRUE as EM_VAL } else { _EMVAL_FALSE as EM_VAL },
        }
    }

    pub fn uses_ref_count(&self) -> bool {
        let last: EM_VAL = unsafe { std::mem::transmute(_EMVAL_LAST_RESERVED_HANDLE) };
        self.handle > last
    }

    pub fn release_ownership(&mut self) -> EM_VAL {
        let h = self.handle;
        self.handle = std::ptr::null_mut();
        h
    }

    pub fn from_fn<F: FnMut(&Val) + 'static>(f: F) -> Self {
        unsafe {
            let a: *mut Box<dyn FnMut(&Val)> = Box::into_raw(Box::new(Box::new(f)));
            let data = a as *mut std::os::raw::c_void;
            Self {
                handle: _emval_take_fn(data),
            }
        }
    }
}

impl Drop for Val {
    fn drop(&mut self) {
        if self.uses_ref_count() {
            unsafe {
                _emval_decref(self.as_handle());
            }
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Clone for Val {
    fn clone(&self) -> Self {
        if self.uses_ref_count() {
            unsafe { _emval_incref(self.handle); }
        }
        Self {
            handle: self.handle,
        }
    }
}

impl From<i32> for Val {
    fn from(item: i32) -> Self {
        Val::from_i32(item)
    }
}

impl From<u32> for Val {
    fn from(item: u32) -> Self {
        Val::from_u32(item)
    }
}

impl From<f32> for Val {
    fn from(item: f32) -> Self {
        Val::from_f32(item)
    }
}

impl From<f64> for Val {
    fn from(item: f64) -> Self {
        Val::from_f64(item)
    }
}

impl From<bool> for Val {
    fn from(item: bool) -> Self {
        Val::from_bool(item)
    }
}

impl From<&str> for Val {
    fn from(item: &str) -> Self {
        Val::from_str(item)
    }
}
