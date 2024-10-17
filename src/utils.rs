use crate::TYPEID;

pub fn get_type_id<T: 'static>() -> TYPEID {
    let v: u128 = unsafe { std::mem::transmute(std::any::TypeId::of::<T>()) };
    v as _
}

static mut NEXT_TYPE_ID: i32 = 1;

pub fn get_next_type_id() -> crate::TYPEID {
    unsafe {
        let id = NEXT_TYPE_ID;
        NEXT_TYPE_ID += 1;
        id as crate::TYPEID
    }
}
