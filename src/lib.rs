#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use emscripten_val_sys::sys;
use std::cmp::Ordering;
use std::ffi::CString;

/// Emscripten's EM_VAL type
#[allow(non_camel_case_types)]
pub type EM_VAL = sys::EM_VAL;

extern "C" {
    pub fn _emval_as_str(v: sys::EM_VAL) -> *mut i8;
    pub fn _emval_add_event_listener(v: sys::EM_VAL, f: *const i8, data: *mut ());
    pub fn _emval_take_fn(data: *const ()) -> EM_VAL;
}

#[no_mangle]
unsafe extern "C" fn emscripten_val_rust_caller(em: sys::EM_VAL, data: *const ()) {
    let mut val = Val::take_ownership(em);
    let a = data as *mut Box<dyn FnMut(&Val)>;
    let f: &mut (dyn FnMut(&Val)) = &mut **a;
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&val)));
    val.release_ownership();
}

/// A helper macro which transforms every argument into a Val object.
/// This helps reduce boilerplate for `Val::call`.
#[macro_export]
macro_rules! argv {
    ($($rest:expr),*) => {{
        &[$(&Val::from($rest)),*]
    }};
}

/// Val is a wrapper around emscripten's EM_VAL type, which itself represents javascript objects
#[repr(C)]
#[derive(Eq)]
pub struct Val {
    handle: sys::EM_VAL,
}

impl Val {
    /// Looks up a global value represented by `name`
    pub fn global(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        Self {
            handle: unsafe { sys::_emval_get_global(name.as_ptr()) },
        }
    }

    /// Creates a Val from a raw handle. This can be used for retrieving values from JavaScript, where the JavaScript side should wrap a value with Emval.toHandle, pass it to Rust, and then Rust can use take_ownership to convert it to a Val instance
    pub fn take_ownership(v: sys::EM_VAL) -> Self {
        Self { handle: v }
    }

    /// Create a Val from another Val instance
    pub fn from_val(v: &Val) -> Self {
        let handle = v.as_handle();
        if v.uses_ref_count() {
            unsafe {
                sys::_emval_incref(handle);
            }
        }
        Self { handle }
    }

    /// Create a Val that represents undefined
    pub fn undefined() -> Self {
        Self {
            handle: sys::_EMVAL_UNDEFINED as EM_VAL,
        }
    }

    /// Creates a new Object
    pub fn object() -> Self {
        Self {
            handle: unsafe { sys::_emval_new_object() },
        }
    }

    /// Create a Val that represents null
    pub fn null() -> Self {
        Self {
            handle: sys::_EMVAL_NULL as EM_VAL,
        }
    }

    /// Creates and returns a new Array
    pub fn array() -> Self {
        Self {
            handle: unsafe { sys::_emval_new_array() },
        }
    }

    /// Creates a Val from a string slice
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let s = CString::new(s).unwrap();
        Self {
            handle: unsafe { sys::_emval_new_cstring(s.as_ptr() as _) },
        }
    }

    /// Looks up a value by the provided name on the Emscripten Module object.
    pub fn module_property(s: &str) -> Self {
        let s = CString::new(s).unwrap();
        Self {
            handle: unsafe { sys::_emval_get_module_property(s.as_ptr() as _) },
        }
    }

    /// Creates a Val from an array
    pub fn from_array<T: Clone + Into<Val>>(arr: &[T]) -> Self {
        let v = Val::array();
        for elem in arr {
            v.call("push", argv![elem.clone().into()]);
        }
        v
    }

    /// Get the EM_VAL handle of a Val object
    pub fn as_handle(&self) -> EM_VAL {
        self.handle
    }

    /// Call a method associated with the JS object represented by the Val object
    pub fn call(&self, f: &str, args: &[&Val]) -> Val {
        unsafe {
            let typeids = vec![sys::EmvalType; args.len() + 1];
            let f = CString::new(f).unwrap();
            let caller =
                sys::_emval_get_method_caller(typeids.len() as u32, typeids.as_ptr() as _, 0);
            for arg in args {
                sys::_emval_incref(arg.handle);
            }
            let ret = sys::_emval_call_method(
                caller,
                self.handle,
                f.as_ptr() as _,
                std::ptr::null_mut(),
                *(args.as_ptr() as *const *const ()) as _,
            );
            let ret = ret as u32 as EM_VAL;
            Val::take_ownership(ret)
        }
    }

    /// Get a property
    pub fn get<T: Clone + Into<Val>>(&self, prop: &T) -> Val {
        let prop: Val = prop.clone().into();
        Val {
            handle: unsafe { sys::_emval_get_property(self.handle, prop.handle) },
        }
    }

    /// Set a property
    pub fn set<T: Clone + Into<Val>, U: Clone + Into<Val>>(&self, prop: &T, val: &U) {
        let prop: Val = prop.clone().into();
        let val: Val = val.clone().into();
        unsafe { sys::_emval_set_property(self.handle, prop.handle, val.handle) };
    }

    /// Generate a Val object from an i32
    pub fn from_i32(i: i32) -> Self {
        Self {
            handle: unsafe { sys::_emval_take_value(sys::IntType, [i as *const ()].as_ptr() as _) },
        }
    }

    /// Generate a Val object from an u32
    pub fn from_u32(i: u32) -> Self {
        Self {
            handle: unsafe { sys::_emval_take_value(sys::IntType, [i as *const ()].as_ptr() as _) },
        }
    }

    /// Generate a Val object from an f32
    pub fn from_f32(i: f32) -> Self {
        let i = i as i32 as *const ();
        Self {
            handle: unsafe { sys::_emval_take_value(sys::FloatType, [i].as_ptr() as _) },
        }
    }

    /// Generate a Val object from an f64
    pub fn from_f64(i: f64) -> Self {
        let i = i as f32 as i32 as *const ();
        Self {
            handle: unsafe { sys::_emval_take_value(sys::FloatType, [i].as_ptr() as _) },
        }
    }

    /// Generate a Val object from a bool
    pub fn from_bool(i: bool) -> Self {
        Self {
            handle: if i {
                sys::_EMVAL_TRUE as EM_VAL
            } else {
                sys::_EMVAL_FALSE as EM_VAL
            },
        }
    }

    /// Checks whether the underlying type uses ref counting
    fn uses_ref_count(&self) -> bool {
        self.handle > sys::_EMVAL_LAST_RESERVED_HANDLE as EM_VAL
    }

    /// Get and release ownership of the internal handle
    pub fn release_ownership(&mut self) -> EM_VAL {
        let h = self.handle;
        self.handle = std::ptr::null_mut();
        h
    }

    /// Checks if the JavaScript object has own (non-inherited) property with the specified name.
    pub fn has_own_property(&self, key: &str) -> bool {
        Val::global("Object")
            .get(&"prototype")
            .get(&"hasOwnProperty")
            .call("call", argv![self.clone(), key])
            .as_bool()
    }

    /// Converts current value to an f64
    pub fn as_f64(&self) -> f64 {
        unsafe { sys::_emval_as(self.handle, sys::FloatType, std::ptr::null_mut()) }
    }

    /// Converts current value to an f32
    pub fn as_f32(&self) -> f32 {
        unsafe { sys::_emval_as(self.handle, sys::FloatType, std::ptr::null_mut()) as f32 }
    }

    /// Converts current value to an i32
    pub fn as_i32(&self) -> i32 {
        unsafe { sys::_emval_as(self.handle, sys::IntType, std::ptr::null_mut()) as i32 }
    }

    /// Converts current value to a u32
    pub fn as_u32(&self) -> u32 {
        unsafe { sys::_emval_as(self.handle, sys::IntType, std::ptr::null_mut()) as u32 }
    }

    /// Converts current value to a bool. This can be useful also to check if a returned object is valid
    pub fn as_bool(&self) -> bool {
        unsafe { sys::_emval_as(self.handle, sys::BoolType, std::ptr::null_mut()) as i32 != 0 }
    }

    /// Converts current value to a string
    pub fn as_string(&self) -> String {
        unsafe {
            let ptr = _emval_as_str(self.handle);
            CString::from_raw(ptr).to_string_lossy().to_string()
        }
    }

    /// Checks whether a value is null
    pub fn is_null(&self) -> bool {
        self.handle == sys::_EMVAL_NULL as EM_VAL
    }

    /// Checks whether a value is undefined
    pub fn is_undefined(&self) -> bool {
        self.handle == sys::_EMVAL_UNDEFINED as EM_VAL
    }

    /// Checks whether a value is true
    pub fn is_true(&self) -> bool {
        self.handle == sys::_EMVAL_TRUE as EM_VAL
    }

    /// Checks whether a value is false
    pub fn is_false(&self) -> bool {
        self.handle == sys::_EMVAL_FALSE as EM_VAL
    }

    /// Checks whether a value is a number
    pub fn is_number(&self) -> bool {
        unsafe { sys::_emval_is_number(self.handle) }
    }

    /// Checks whether a value is a string
    pub fn is_string(&self) -> bool {
        unsafe { sys::_emval_is_string(self.handle) }
    }

    /// Checks whether the object is an instanceof another object
    pub fn instance_of(&self, v: &Val) -> bool {
        unsafe { sys::_emval_instanceof(self.as_handle(), v.as_handle()) }
    }

    /// Checks whether a value is an Array
    pub fn is_array(&self) -> bool {
        self.instance_of(&Val::global("Array"))
    }

    /// Checks if the specified property is in the specified object
    pub fn is_in(&self, v: &Val) -> bool {
        unsafe { sys::_emval_in(self.as_handle(), v.as_handle()) }
    }

    /// Returns the typeof the object
    pub fn type_of(&self) -> Val {
        Val {
            handle: unsafe { sys::_emval_typeof(self.handle) },
        }
    }

    /// Throw the object as a JS exception
    pub fn throw(&self) -> bool {
        unsafe { sys::_emval_throw(self.as_handle()) }
    }

    /// Pauses the Rust code to await the Promise / thenable. This requires [ASYNCIFY](https://emscripten.org/docs/tools_reference/settings_reference.html#asyncify) to be enabled
    pub fn await_(&self) -> Val {
        Val {
            handle: unsafe { sys::_emval_await(self.handle) },
        }
    }

    /// Removes a property from an object
    pub fn delete<T: Clone + Into<Val>>(&self, prop: &T) -> bool {
        unsafe { sys::_emval_delete(self.as_handle(), prop.clone().into().as_handle()) }
    }

    /// Instantiate a new object, passes the `args` to the object's contructor
    pub fn new(&self, args: &[&Val]) -> Val {
        unsafe {
            let typeids = vec![sys::EmvalType; args.len() + 1];
            let caller =
                sys::_emval_get_method_caller(typeids.len() as u32, typeids.as_ptr() as _, 1);
            for arg in args {
                sys::_emval_incref(arg.handle);
            }
            let ret = sys::_emval_call(
                caller,
                self.handle,
                std::ptr::null_mut(),
                *(args.as_ptr() as *const *const ()) as _,
            );
            let ret = ret as u32 as EM_VAL;
            Val::take_ownership(ret)
        }
    }

    fn gt<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { sys::_emval_greater_than(self.handle, v.clone().into().handle) }
    }

    fn lt<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { sys::_emval_less_than(self.handle, v.clone().into().handle) }
    }

    fn equals<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { sys::_emval_equals(self.handle, v.clone().into().handle) }
    }

    /// Check if the current object is strictly equals to another object `===`
    pub fn strictly_equals<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { sys::_emval_strictly_equals(self.handle, v.clone().into().handle) }
    }

    /// Checks the validity of an object
    pub fn not(&self) -> bool {
        unsafe { sys::_emval_not(self.handle) }
    }

    /// Convenience method.
    /// Adds a callback to an EventTarget object
    pub fn add_event_listener<F: FnMut(&Val) + 'static>(&self, ev: &str, f: F) {
        unsafe {
            let a: *mut Box<dyn FnMut(&Val)> = Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let ev = CString::new(ev).unwrap();
            _emval_add_event_listener(self.handle, ev.as_ptr() as _, data as _);
        }
    }

    /// Generates a Val object from a function object
    pub fn from_fn<F: FnMut(&Val) + 'static>(f: F) -> Val {
        unsafe {
            let a: *mut Box<dyn FnMut(&Val)> = Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            Self {
                handle: _emval_take_fn(data as _),
            }
        }
    }
}

impl Drop for Val {
    fn drop(&mut self) {
        if self.uses_ref_count() {
            unsafe {
                sys::_emval_decref(self.as_handle());
            }
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Clone for Val {
    fn clone(&self) -> Self {
        if self.uses_ref_count() {
            unsafe {
                sys::_emval_incref(self.handle);
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
