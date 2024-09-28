use std::ffi::CString;
use std::os::raw::*;

pub mod sys {
    pub use emscripten_val_sys::sys::*;
}

use sys::*;

pub type EventListenerCb = Option<unsafe extern "C" fn(EM_VAL, *mut c_void)>;

#[macro_export]
macro_rules! argv {
    ($($rest:expr),*) => {{
        &[$(Val::from($rest).as_ptr()),*]
    }};
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

    pub fn as_ptr(&self) -> *const c_void {
        unsafe { std::mem::transmute(self.clone()) }
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

    pub fn from_numeric_array<T: Clone + Into<Val>>(arr: &[T]) -> Self {
        let v = Val::array();
        for elem in arr {
            unsafe {
                v.call("push", argv![elem.clone().into()]);
            }
        }
        v
    }

    // pub fn from_val_array(arr: &[Val]) -> Self {
    //     let v = Val::array();
    //     for elem in arr {
    //         unsafe {
    //             v.call("push", argv![elem.clone()]);
    //         }
    //     }
    //     v
    // }

    pub fn as_handle(&self) -> EM_VAL {
        self.handle
    }

    pub unsafe fn call0(&self, f: &str, args: (Vec<TYPEID>, Vec<*const c_void>)) -> Val {
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

    pub unsafe fn call(&self, f: &str, args: &[*const c_void]) -> Val {
        let f = CString::new(f).unwrap();
        let typeids = vec![EmvalType; args.len() + 1];
        let caller = _emval_get_method_caller(typeids.len() as u32, typeids.as_ptr() as _, 0);
        let ret = _emval_call_method(
            caller,
            self.handle,
            f.as_ptr() as _,
            std::ptr::null_mut(),
            args.as_ptr() as _,
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

// impl Into<Val> for i32 {
//     fn into(self) -> Val {
//         Val::from(self)
//     }
// }