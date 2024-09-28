use emscripten_val::*;

fn main() {
    let a = Val::from_numeric_array(&[1, 2]);
    unsafe {
        a.call("push", argv![1]);
    }

    let console = Val::global("console");
    unsafe {
        console.call("log", argv![a]);
    }

    let document = Val::global("document");
    unsafe {
        let elem = document.call("createElement", argv!["BUTTON"]);
        elem.set(&Val::from("textContent"), &Val::from("Click"));
        let bodys = document.call("getElementsByTagName", argv!["body"]);
        let body = bodys.at(0);
        console.call("clear", argv![]);
        body.call("appendChild", argv![elem]);
    }
}
