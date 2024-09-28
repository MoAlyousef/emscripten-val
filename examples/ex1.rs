use emscripten_val::*;

fn main() {
    let a = Val::from_numeric_array(&[1, 2]);
    unsafe {
        a.call("push", gen_args![1]);
    }

    let console = Val::global("console");
    unsafe {
        console.call("log", gen_args![a]);
    }

    let document = Val::global("document");
    unsafe {
        let elem = document.call("createElement", gen_args![Val::from("BUTTON")]);
        elem.set(&Val::from("textContent"), &Val::from("Click"));
        let bodys = document.call("getElementsByTagName", gen_args![Val::from("body")]);
        let body = bodys.at(0);
        body.call("appendChild", gen_args![elem.clone()]);
        // console.call("clear", gen_args![]);
        elem.call("addEventListener", gen_args![Val::from("click"), Val::from_fn(|v: &Val| {
            dbg!("Here");
        })]);
    }
}
