use crate::Val;
use emscripten_val_sys::sys;
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

impl From<()> for Val {
    fn from(_: ()) -> Self {
        Val::null()
    }
}

impl From<i32> for Val {
    fn from(item: i32) -> Self {
        Val::from_i32(item)
    }
}

impl From<Val> for i32 {
    fn from(item: Val) -> Self {
        item.as_i32()
    }
}

impl From<u32> for Val {
    fn from(item: u32) -> Self {
        Val::from_u32(item)
    }
}

impl From<Val> for u32 {
    fn from(item: Val) -> Self {
        item.as_u32()
    }
}

impl From<f32> for Val {
    fn from(item: f32) -> Self {
        Val::from_f32(item)
    }
}

impl From<Val> for f32 {
    fn from(item: Val) -> Self {
        item.as_f32()
    }
}

impl From<f64> for Val {
    fn from(item: f64) -> Self {
        Val::from_f64(item)
    }
}

impl From<Val> for f64 {
    fn from(item: Val) -> Self {
        item.as_f64()
    }
}

impl From<bool> for Val {
    fn from(item: bool) -> Self {
        Val::from_bool(item)
    }
}

impl From<Val> for bool {
    fn from(item: Val) -> Self {
        item.as_bool()
    }
}

impl From<&str> for Val {
    fn from(item: &str) -> Self {
        Val::from_str(item)
    }
}

impl From<Val> for String {
    fn from(item: Val) -> Self {
        item.as_string()
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
