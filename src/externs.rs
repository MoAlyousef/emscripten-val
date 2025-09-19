use crate::{val::Val, val::EM_VAL};

extern "C" {
    pub fn _emval_as_str(v: EM_VAL) -> *mut i8;
    pub fn _emval_as_bytes(v: EM_VAL, output_buffer: *mut u8) -> u64;
    pub fn _emval_add_event_listener(v: EM_VAL, f: *const i8, data: *mut ());
    pub fn _emval_take_fn(argcount: u8, data: *const ()) -> EM_VAL;
    pub fn _emval_call_method_raw(
        object: EM_VAL,
        method: *const i8,
        argv: *const EM_VAL,
        argc: i32,
    ) -> EM_VAL;
    pub fn _emval_construct_raw(constructor: EM_VAL, argv: *const EM_VAL, argc: i32) -> EM_VAL;
    // Raw function call (fn.apply(undefined, args)) used as a debug fallback.
    pub fn _emval_call_function_raw(function: EM_VAL, argv: *const EM_VAL, argc: i32) -> EM_VAL;
    pub fn free(ptr: *const ());
}

#[no_mangle]
unsafe extern "C" fn emscripten_val_rust_caller0(data: *const ()) -> EM_VAL {
    let a = data as *mut Box<dyn FnMut() -> Val>;
    let f: &mut (dyn FnMut() -> Val) = &mut **a;
    let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    if let Ok(mut ret) = ret {
        ret.release_ownership()
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn emscripten_val_rust_caller1(em: EM_VAL, data: *const ()) -> EM_VAL {
    let mut val = Val::take_ownership(em);
    let a = data as *mut Box<dyn FnMut(&Val) -> Val>;
    let f: &mut (dyn FnMut(&Val) -> Val) = &mut **a;
    let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&val)));
    val.release_ownership();
    if let Ok(mut ret) = ret {
        ret.release_ownership()
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn emscripten_val_rust_caller2(
    a0: EM_VAL,
    a1: EM_VAL,
    data: *const (),
) -> EM_VAL {
    let mut val0 = Val::take_ownership(a0);
    let mut val1 = Val::take_ownership(a1);
    let a = data as *mut Box<dyn FnMut(&Val, &Val) -> Val>;
    let f: &mut (dyn FnMut(&Val, &Val) -> Val) = &mut **a;
    let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&val0, &val1)));
    val0.release_ownership();
    val1.release_ownership();
    if let Ok(mut ret) = ret {
        ret.release_ownership()
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn emscripten_val_rust_caller3(
    a0: EM_VAL,
    a1: EM_VAL,
    a2: EM_VAL,
    data: *const (),
) -> EM_VAL {
    let mut val0 = Val::take_ownership(a0);
    let mut val1 = Val::take_ownership(a1);
    let mut val2 = Val::take_ownership(a2);
    let a = data as *mut Box<dyn FnMut(&Val, &Val, &Val) -> Val>;
    let f: &mut (dyn FnMut(&Val, &Val, &Val) -> Val) = &mut **a;
    let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&val0, &val1, &val2)));
    val0.release_ownership();
    val1.release_ownership();
    val2.release_ownership();
    if let Ok(mut ret) = ret {
        ret.release_ownership()
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn emscripten_val_rust_caller4(
    a0: EM_VAL,
    a1: EM_VAL,
    a2: EM_VAL,
    a3: EM_VAL,
    data: *const (),
) -> EM_VAL {
    let mut val0 = Val::take_ownership(a0);
    let mut val1 = Val::take_ownership(a1);
    let mut val2 = Val::take_ownership(a2);
    let mut val3 = Val::take_ownership(a3);
    let a = data as *mut Box<dyn FnMut(&Val, &Val, &Val, &Val) -> Val>;
    let f: &mut (dyn FnMut(&Val, &Val, &Val, &Val) -> Val) = &mut **a;
    let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        f(&val0, &val1, &val2, &val3)
    }));
    val0.release_ownership();
    val1.release_ownership();
    val2.release_ownership();
    val3.release_ownership();
    if let Ok(mut ret) = ret {
        ret.release_ownership()
    } else {
        std::ptr::null_mut()
    }
}
