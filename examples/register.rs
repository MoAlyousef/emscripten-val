use emscripten_val::{JsType, *};

#[repr(C)]
#[derive(Default, Clone)]
struct MyRustClass {
    v: i32,
}

impl JsType for MyRustClass {
    fn id() -> crate::TYPEID {
        utils::get_type_id::<MyRustClass>()
    }

    fn from_generic_wire_type(v: GenericWireType) -> Self {
        let ptr = v.0 as usize as *const MyRustClass;
        if ptr.is_null() {
            Default::default()
        } else {
            unsafe { (*ptr).clone() }
        }
    }
}

fn main() {
        register_class::<MyRustClass>("MyRustClass");
        register_class_default_constructor::<MyRustClass>();
        register_class_property!(
            MyRustClass,
            "val",
            v,
            i32
        );


    let global = Val::global("window");

    global.set(&"myclass", &MyRustClass { v: 42 });
    let myclass = global.get(&"myclass");
    println!("{}", myclass.as_::<MyRustClass>().v);
}
