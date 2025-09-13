use emscripten_val_sys::val;
use std::ffi::{CStr, CString};

use crate::externs::*;
use crate::id::JsType;

/// Emscripten's EM_VAL type
#[allow(non_camel_case_types)]
pub type EM_VAL = val::EM_VAL;

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
    handle: EM_VAL,
}

impl Val {
    fn id() -> val::TYPEID {
        extern "C" {
            fn EmvalType() -> val::TYPEID;
        }
        unsafe { EmvalType() }
    }
    /// Looks up a global value represented by `name`
    pub fn global(name: &str) -> Self {
        let name = CString::new(name).unwrap();
        Self {
            handle: unsafe { val::_emval_get_global(name.as_ptr()) },
        }
    }

    /// Creates a Val from a raw handle. This can be used for retrieving values from JavaScript, where the JavaScript side should wrap a value with Emval.toHandle, pass it to Rust, and then Rust can use take_ownership to convert it to a Val instance
    pub fn take_ownership(v: val::EM_VAL) -> Self {
        Self { handle: v }
    }

    /// Create a Val from another Val instance
    pub fn from_val(v: &Val) -> Self {
        let handle = v.as_handle();
        if v.uses_ref_count() {
            unsafe {
                val::_emval_incref(handle);
            }
        }
        Self { handle }
    }

    /// Create a Val that represents undefined
    pub fn undefined() -> Self {
        Self {
            handle: val::_EMVAL_UNDEFINED as EM_VAL,
        }
    }

    /// Creates a new Object
    pub fn object() -> Self {
        Self {
            handle: unsafe { val::_emval_new_object() },
        }
    }

    /// Create a Val that represents null
    pub fn null() -> Self {
        Self {
            handle: val::_EMVAL_NULL as EM_VAL,
        }
    }

    /// Creates and returns a new Array
    pub fn array() -> Self {
        Self {
            handle: unsafe { val::_emval_new_array() },
        }
    }

    /// Creates a Val from a string slice
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let s = CString::new(s).unwrap();
        Self {
            handle: unsafe { val::_emval_new_cstring(s.as_ptr() as _) },
        }
    }

    /// Looks up a value by the provided name on the Emscripten Module object.
    pub fn module_property(s: &str) -> Self {
        let s = CString::new(s).unwrap();
        Self {
            handle: unsafe { val::_emval_get_module_property(s.as_ptr() as _) },
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
            let typeids = vec![Val::id(); args.len() + 1];
            let f = CString::new(f).unwrap();
            let caller = val::_emval_create_invoker(
                typeids.len() as u32, 
                typeids.as_ptr() as _, 
                val::EM_INVOKER_KIND_METHOD
            );
            
            for arg in args {
                val::_emval_incref(arg.handle);
            }

            let ret = val::_emval_invoke(
                caller,
                self.handle,
                f.as_ptr() as _,
                std::ptr::null_mut(),
                *(args.as_ptr() as *const *const ()) as _,
            );
            
            // For Val return types, the wire type contains the handle encoded as double
            let ret_wire = crate::id::GenericWireType(ret);
            let ret_handle = ret_wire.0 as usize as EM_VAL;
            
            // Check for reserved handles - these don't need reference counting
            let handle_value = ret_handle as usize;
            if handle_value <= val::_EMVAL_LAST_RESERVED_HANDLE as usize {
                // Reserved handle - use directly without take_ownership
                Val { handle: ret_handle }
            } else {
                // Regular handle - use take_ownership
                Val::take_ownership(ret_handle)
            }
        }
    }

    /// Get a property
    pub fn get<T: Clone + Into<Val>>(&self, prop: &T) -> Val {
        let prop: Val = prop.clone().into();
        Val {
            handle: unsafe { val::_emval_get_property(self.handle, prop.handle) },
        }
    }

    /// Set a property
    pub fn set<T: Clone + Into<Val>, U: Clone + Into<Val>>(&self, prop: &T, val: &U) {
        let prop: Val = prop.clone().into();
        let val: Val = val.clone().into();
        unsafe { val::_emval_set_property(self.handle, prop.handle, val.handle) };
    }

    /// Generate a Val object from a type implementing JsType
    pub fn from_<T: JsType>(v: T) -> Self {
        unsafe {
            // For pointer-like/user types (default signature 'p'), embind expects a
            // pointer value read from memory (i.e., argv points to a location that
            // contains the raw pointer). To ensure correct lifetime, allocate on the heap
            // and pass a pointer to that pointer. For primitive types ('i', 'd'), pass
            // a pointer to the value directly.
            let handle = match T::signature() {
                'p' => {
                    let boxed = Box::new(v);
                    let mut ptr: *mut T = Box::into_raw(boxed);
                    val::_emval_take_value(T::id(), (&mut ptr as *mut *mut T) as _)
                }
                _ => val::_emval_take_value(T::id(), (&v as *const T) as _),
            };
            Self { handle }
        }
    }

    /// Generate a Val object from a type implementing JsType
    pub fn as_<T: JsType>(&self) -> T {
        unsafe {
            T::from_generic_wire_type(crate::id::GenericWireType(val::_emval_as(
                self.handle,
                T::id(),
                std::ptr::null_mut(),
            )))
        }
    }

    /// Generate a Val object from a type implementing JsType
    pub fn as_i32(&self) -> i32 {
        unsafe { val::_emval_as(self.handle, i32::id(), std::ptr::null_mut()) as i32 }
    }

    /// Checks whether the underlying type uses ref counting
    fn uses_ref_count(&self) -> bool {
        self.handle > val::_EMVAL_LAST_RESERVED_HANDLE as EM_VAL
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
            .as_::<bool>()
    }

    /// Checks whether a value is null
    pub fn is_null(&self) -> bool {
        self.handle == val::_EMVAL_NULL as EM_VAL
    }

    /// Checks whether a value is undefined
    pub fn is_undefined(&self) -> bool {
        self.handle == val::_EMVAL_UNDEFINED as EM_VAL
    }

    /// Checks whether a value is true
    pub fn is_true(&self) -> bool {
        self.handle == val::_EMVAL_TRUE as EM_VAL
    }

    /// Checks whether a value is false
    pub fn is_false(&self) -> bool {
        self.handle == val::_EMVAL_FALSE as EM_VAL
    }

    /// Checks whether a value is a number
    pub fn is_number(&self) -> bool {
        unsafe { val::_emval_is_number(self.handle) }
    }

    /// Checks whether a value is a string
    pub fn is_string(&self) -> bool {
        unsafe { val::_emval_is_string(self.handle) }
    }

    /// Checks whether the object is an instanceof another object
    pub fn instanceof(&self, v: &Val) -> bool {
        unsafe { val::_emval_instanceof(self.as_handle(), v.as_handle()) }
    }

    /// Checks whether a value is an Array
    pub fn is_array(&self) -> bool {
        self.instanceof(&Val::global("Array"))
    }

    /// Checks if the specified property is in the specified object
    pub fn is_in(&self, v: &Val) -> bool {
        unsafe { val::_emval_in(self.as_handle(), v.as_handle()) }
    }

    /// Returns the typeof the object
    pub fn type_of(&self) -> Val {
        Val {
            handle: unsafe { val::_emval_typeof(self.handle) },
        }
    }

    /// Throw the object as a JS exception
    pub fn throw(&self) -> bool {
        unsafe { val::_emval_throw(self.as_handle()) }
    }

    /// Pauses the Rust code to await the Promise / thenable. This requires [ASYNCIFY](https://emscripten.org/docs/tools_reference/settings_reference.html#asyncify) to be enabled
    pub fn await_(&self) -> Val {
        Val {
            handle: unsafe { val::_emval_await(self.handle) },
        }
    }

    /// Removes a property from an object
    pub fn delete<T: Clone + Into<Val>>(&self, prop: &T) -> bool {
        unsafe { val::_emval_delete(self.as_handle(), prop.clone().into().as_handle()) }
    }

    /// Instantiate a new object, passes the `args` to the object's contructor
    pub fn new(&self, args: &[&Val]) -> Val {
        unsafe {
            let typeids = vec![Val::id(); args.len() + 1];
            let caller = val::_emval_create_invoker(
                typeids.len() as u32, 
                typeids.as_ptr() as _, 
                val::EM_INVOKER_KIND_CONSTRUCTOR
            );
            for arg in args {
                val::_emval_incref(arg.handle);
            }
            
            let ret = val::_emval_invoke(
                caller,
                self.handle,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                *(args.as_ptr() as *const *const ()) as _,
            );
            
            // For Val return types, the wire type contains the handle encoded as double
            let ret_wire = crate::id::GenericWireType(ret);
            let ret_handle = ret_wire.0 as usize as EM_VAL;
            
            // Check for reserved handles - these don't need reference counting
            let handle_value = ret_handle as usize;
            if handle_value <= val::_EMVAL_LAST_RESERVED_HANDLE as usize {
                // Reserved handle - use directly without take_ownership
                Val { handle: ret_handle }
            } else {
                // Regular handle - use take_ownership
                Val::take_ownership(ret_handle)
            }
        }
    }

    fn gt<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { val::_emval_greater_than(self.handle, v.clone().into().handle) }
    }

    fn lt<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { val::_emval_less_than(self.handle, v.clone().into().handle) }
    }

    fn equals<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { val::_emval_equals(self.handle, v.clone().into().handle) }
    }

    /// Check if the current object is strictly equals to another object `===`
    pub fn strictly_equals<T: Clone + Into<Val>>(&self, v: &T) -> bool {
        unsafe { val::_emval_strictly_equals(self.handle, v.clone().into().handle) }
    }

    /// Checks the validity of an object
    pub fn not(&self) -> bool {
        unsafe { val::_emval_not(self.handle) }
    }

    /// Convenience method.
    /// Adds a callback to an EventTarget object
    pub fn add_event_listener<F: (FnMut(&Val) -> Val) + 'static>(&self, ev: &str, f: F) {
        unsafe {
            let a: *mut Box<dyn FnMut(&Val) -> Val> = Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let ev = CString::new(ev).unwrap();
            _emval_add_event_listener(self.handle, ev.as_ptr() as _, data as _);
        }
    }

    /// Generates a Val object from a function object which takes 0 args
    pub fn from_fn0<F: (FnMut() -> Val) + 'static>(f: F) -> Val {
        unsafe {
            let a: *mut Box<dyn FnMut() -> Val> = Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            Self {
                handle: _emval_take_fn(0, data as _),
            }
        }
    }

    /// Generates a Val object from a function object which takes 1 arg
    pub fn from_fn1<F: (FnMut(&Val) -> Val) + 'static>(f: F) -> Val {
        unsafe {
            #[allow(clippy::type_complexity)]
            let a: *mut Box<dyn FnMut(&Val) -> Val> = Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            Self {
                handle: _emval_take_fn(1, data as _),
            }
        }
    }

    /// Generates a Val object from a function object which takes 2 args
    pub fn from_fn2<F: (FnMut(&Val, &Val) -> Val) + 'static>(f: F) -> Val {
        unsafe {
            #[allow(clippy::type_complexity)]
            let a: *mut Box<dyn FnMut(&Val, &Val) -> Val> = Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            Self {
                handle: _emval_take_fn(2, data as _),
            }
        }
    }

    /// Generates a Val object from a function object which takes 3 args
    pub fn from_fn3<F: (FnMut(&Val, &Val, &Val) -> Val) + 'static>(f: F) -> Val {
        unsafe {
            #[allow(clippy::type_complexity)]
            let a: *mut Box<dyn FnMut(&Val, &Val, &Val) -> Val> =
                Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            Self {
                handle: _emval_take_fn(3, data as _),
            }
        }
    }

    /// Generates a Val object from a function object which takes 4 args
    pub fn from_fn4<F: (FnMut(&Val, &Val, &Val, &Val) -> Val) + 'static>(f: F) -> Val {
        unsafe {
            #[allow(clippy::type_complexity)]
            let a: *mut Box<dyn FnMut(&Val, &Val, &Val, &Val) -> Val> =
                Box::into_raw(Box::new(Box::new(f)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            Self {
                handle: _emval_take_fn(4, data as _),
            }
        }
    }

    /// Converts current value to a string
    pub fn as_string(&self) -> String {
        unsafe {
            let ptr = _emval_as_str(self.handle);
            let ret = CStr::from_ptr(ptr).to_string_lossy().to_string();
            free(ptr as _);
            ret
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        unsafe {
            let len = _emval_as_bytes(self.handle, std::ptr::null_mut()) as usize;
            let mut output_buffer: Vec<u8> = vec![0; len];
            let _ = _emval_as_bytes(self.handle, output_buffer.as_mut_ptr());
            output_buffer
        }
    }

    /// Convert a javascript Array to a Rust Vec
    pub fn to_vec<T: JsType>(&self) -> Vec<T> {
        let len = self.get(&"length").as_::<u32>();
        let mut v: Vec<T> = vec![];
        for i in 0..len {
            v.push(self.get(&i).as_::<T>());
        }
        v
    }
}

use std::cmp::Ordering;

impl Default for Val {
    fn default() -> Val {
        Val::null()
    }
}

impl Drop for Val {
    fn drop(&mut self) {
        if self.uses_ref_count() {
            unsafe {
                val::_emval_decref(self.as_handle());
            }
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Clone for Val {
    fn clone(&self) -> Self {
        if self.uses_ref_count() {
            unsafe {
                val::_emval_incref(self.handle);
            }
        }
        Self {
            handle: self.handle,
        }
    }
}

impl<T: JsType> From<T> for Val {
    fn from(v: T) -> Self {
        Val::from_(v)
    }
}

impl From<()> for Val {
    fn from(_: ()) -> Self {
        Val::null()
    }
}

impl From<&Val> for Val {
    fn from(item: &Val) -> Self {
        Val::from_val(item)
    }
}

impl From<&str> for Val {
    fn from(item: &str) -> Self {
        Val::from_str(item)
    }
}

impl From<String> for Val {
    fn from(item: String) -> Self {
        Val::from_str(&item)
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
