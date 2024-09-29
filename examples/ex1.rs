use emscripten_val::*;

fn main() {
    Val::module_property("MyBindings");
    let a = Val::from_array(&[1, 2]);
    unsafe {
        a.call("push", argv![1]);
    }

    let console = Val::global("console");
    unsafe {
        console.call("log", argv![a.clone()]);
    }

    let document = Val::global("document");
    unsafe {
        let elem = document.call("createElement", argv!["BUTTON"]);
        elem.set(&"textContent", &"Click");
        let bodys = document.call("getElementsByTagName", argv!["body"]);
        let body = bodys.get(&0);
        console.call("clear", argv![]);
        body.call("appendChild", argv![elem.clone()]);
        elem.call("addEventListener", &[Val::from("click").as_ptr(), Val::from_fn(|v: Val| {}).as_ptr(), Val::from(true).as_ptr()]);
    }

    let arr = Val::from_array(&[a, console.clone(), document]);
    unsafe {
        console.call("log", argv![arr]);
    }
}
