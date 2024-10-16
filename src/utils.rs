use crate::TYPEID;

pub fn get_type_id<T: 'static>() -> TYPEID {
    let v: u128 = unsafe { std::mem::transmute(std::any::TypeId::of::<T>()) };
    v as _
}
