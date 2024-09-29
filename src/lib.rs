use std::cmp::Ordering;
use std::ffi::CString;
use std::mem;

pub mod sys {
    pub use emscripten_val_sys::sys::*;
}

use sys::*;

#[macro_export]
macro_rules! argv {
    ($($rest:expr),*) => {{
        &[$(Val::from($rest).as_ptr()),*]
    }};
}

#[repr(C)]
#[derive(Eq)]
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

    pub fn as_ptr(&self) -> *const () {
        unsafe { mem::transmute(self.clone()) }
    }

    pub fn take_ownership(v: EM_VAL) -> Self {
        Self { handle: v }
    }

    pub fn from_val(v: &Val) -> Self {
        let handle = v.as_handle();
        if v.uses_ref_count() {
            unsafe {
                _emval_incref(handle);
            }
        }
        Self { handle }
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

    pub fn from_array<T: Clone + Into<Val>>(arr: &[T]) -> Self {
        let v = Val::array();
        for elem in arr {
            unsafe {
                v.call("push", argv![elem.clone().into()]);
            }
        }
        v
    }

    pub fn as_handle(&self) -> EM_VAL {
        self.handle
    }

    pub unsafe fn call0(&self, f: &str, args: (Vec<TYPEID>, Vec<*const ()>)) -> Val {
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

    pub unsafe fn call(&self, f: &str, args: &[*const ()]) -> Val {
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

    pub fn get<T: Clone + Into<Val>>(&self, prop: &T) -> Val {
        let prop: Val = prop.clone().into();
        Val {
            handle: unsafe { _emval_get_property(self.handle, prop.handle) },
        }
    }

    pub fn set<T: Clone + Into<Val>, U: Clone + Into<Val>>(&self, prop: &T, val: &U) {
        let prop: Val = prop.clone().into();
        let val: Val = val.clone().into();
        unsafe { _emval_set_property(self.handle, prop.handle, val.handle) };
    }

    pub fn from_i32(i: i32) -> Self {
        // TODO: check val_ref
        Self {
            handle: unsafe { _emval_take_value(IntType, [i as *const ()].as_ptr() as _) },
        }
    }

    pub fn from_u32(i: u32) -> Self {
        Self {
            handle: unsafe { _emval_take_value(IntType, [i as *const ()].as_ptr() as _) },
        }
    }

    pub fn from_f32(i: f32) -> Self {
        let i: *const () = unsafe { mem::transmute(i) };
        Self {
            handle: unsafe { _emval_take_value(FloatType, [i].as_ptr() as _) },
        }
    }

    pub fn from_f64(i: f64) -> Self {
        let i: *const () = unsafe { mem::transmute(i as f32) };
        Self {
            handle: unsafe { _emval_take_value(FloatType, [i].as_ptr() as _) },
        }
    }

    pub fn from_bool(i: bool) -> Self {
        Self {
            handle: if i {
                _EMVAL_TRUE as EM_VAL
            } else {
                _EMVAL_FALSE as EM_VAL
            },
        }
    }

    pub fn uses_ref_count(&self) -> bool {
        let last: EM_VAL = unsafe { mem::transmute(_EMVAL_LAST_RESERVED_HANDLE) };
        self.handle > last
    }

    pub fn release_ownership(&mut self) -> EM_VAL {
        let h = self.handle;
        self.handle = std::ptr::null_mut();
        h
    }

    pub fn has_own_property(&self, key: &str) -> bool {
        unsafe {
            Val::global("Object")
                .get(&"prototype")
                .get(&"hasOwnProperty")
                .call("call", argv![self.clone(), key])
                .as_bool()
        }
    }

    pub fn as_f64(&self) -> f64 {
        unsafe { _emval_as(self.handle, FloatType, std::ptr::null_mut()) }
    }

    pub fn as_f32(&self) -> f32 {
        unsafe { _emval_as(self.handle, FloatType, std::ptr::null_mut()) as f32 }
    }

    pub fn as_i32(&self) -> i32 {
        unsafe { _emval_as(self.handle, IntType, std::ptr::null_mut()) as i32 }
    }

    pub fn as_u32(&self) -> u32 {
        unsafe { _emval_as(self.handle, IntType, std::ptr::null_mut()) as u32 }
    }

    pub fn as_bool(&self) -> bool {
        unsafe { _emval_as(self.handle, BoolType, std::ptr::null_mut()) as i32 != 0 }
    }

    pub fn as_string(&self) -> String {
        unsafe {
            let ptr = _emval_as_str(self.handle);
            CString::from_raw(ptr).to_string_lossy().to_string()
        }
    }

    pub fn is_null(&self) -> bool {
        self.handle == _EMVAL_NULL as EM_VAL
    }

    pub fn is_undefined(&self) -> bool {
        self.handle == _EMVAL_UNDEFINED as EM_VAL
    }

    pub fn is_true(&self) -> bool {
        self.handle == _EMVAL_TRUE as EM_VAL
    }

    pub fn is_false(&self) -> bool {
        self.handle == _EMVAL_FALSE as EM_VAL
    }

    pub fn is_number(&self) -> bool {
        unsafe { _emval_is_number(self.handle) }
    }

    pub fn is_string(&self) -> bool {
        unsafe { _emval_is_string(self.handle) }
    }

    pub fn instanceof(&self, v: &Val) -> bool {
        unsafe { _emval_instanceof(self.as_handle(), v.as_handle()) }
    }

    pub fn is_array(&self) -> bool {
        self.instanceof(&Val::global("Array"))
    }

    pub fn is_in(&self, v: &Val) -> bool {
        unsafe { _emval_in(self.as_handle(), v.as_handle()) }
    }

    pub fn type_of(&self) -> Val {
        Val {
            handle: unsafe { _emval_typeof(self.handle) },
        }
    }

    pub fn throw(&self) -> bool {
        unsafe { _emval_throw(self.as_handle()) }
    }

    pub fn await_(&self) -> Val {
        Val {
            handle: unsafe { _emval_await(self.handle) },
        }
    }

    pub fn delete<T: Clone + Into<Val>>(&self, prop: &T) -> bool {
        unsafe { _emval_delete(self.as_handle(), prop.clone().into().as_handle()) }
    }

    pub fn new(&self, args: &[*const ()]) -> Val {
        unsafe {
            let typeids = vec![EmvalType; args.len() + 1];
            let caller = _emval_get_method_caller(typeids.len() as u32, typeids.as_ptr() as _, 1);
            let ret = _emval_call(
                caller,
                self.handle,
                std::ptr::null_mut(),
                args.as_ptr() as _,
            );
            let ret = ret as u32 as EM_VAL;
            Val::take_ownership(ret)
        }
    }

    fn gt<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { _emval_greater_than(self.handle, v.clone().into().handle) }
    }

    fn lt<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { _emval_less_than(self.handle, v.clone().into().handle) }
    }

    fn equals<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { _emval_equals(self.handle, v.clone().into().handle) }
    }

    pub fn strictly_equals<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { _emval_strictly_equals(self.handle, v.clone().into().handle) }
    }

    pub fn not(&self) -> bool {
        unsafe { _emval_not(self.handle) }
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
            unsafe {
                _emval_incref(self.handle);
            }
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

impl From<&Val> for Val {
    fn from(item: &Val) -> Self {
        Val::from_val(item)
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Val) -> bool {
        self.equals(other)
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Val) -> Option<Ordering> {
        if self.equals(other) {
            Some(Ordering::Equal)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Val) -> Ordering {
        self.partial_cmp(other).expect("Vals incomparable!")
    }
}
