use emscripten_val::{JsType, *};

#[repr(C)]
#[derive(Default, Clone)]
struct MyClass {
    v: i32,
}

impl JsType for MyClass {
    fn id() -> TYPEID {
        utils::get_type_id::<MyClass>()
    }

    fn from_generic_wire_type(v: GenericWireType) -> Self {
        unsafe {
            let ptr = v.0 as usize as *const usize;
            std::mem::transmute(ptr)
        }
    }
}

fn main() {
    register_class::<MyClass>("MyClass");
    register_class_default_constructor::<MyClass>();
    register_class_property!(MyClass, "val", v, i32);
    let global = Val::global("window");

    global.set(&"myclass", &MyClass { v: 42 });
    let myclass = global.get(&"myclass");
    println!("{}", myclass.as_::<MyClass>().v);
}
