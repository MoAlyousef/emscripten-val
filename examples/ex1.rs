use emscripten_val::*;

fn main() {
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
        elem.set("textContent", "Click");
        let bodys = document.call("getElementsByTagName", argv!["body"]);
        let body = bodys.get(0);
        console.call("clear", argv![]);
        body.call("appendChild", argv![elem]);
    }

    let arr = Val::from_array(&[a, console.clone(), document]);
    unsafe {
        console.call("log", argv![arr]);
    }
}
