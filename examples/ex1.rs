use emscripten_val::*;

fn main() {
    let a = Val::from_numeric_array(&[1, 2]);
    unsafe {
        a.call("push", gen_args![1,]);
    }

    let console = Val::global("console");
    unsafe {
        console.call("log", gen_args![a,]);
    }

    let document = Val::global("document");
    unsafe {
        let elem = document.call("createElement", gen_args![Val::from_str("DIV"),]);
        let bodys = document.call("getElementsByTagName", gen_args![Val::from_str("body"),]);
        let body = bodys.at(0);
        body.call("appendChild", gen_args![elem,]);
    }
}
