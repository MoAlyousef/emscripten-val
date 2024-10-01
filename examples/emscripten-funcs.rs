use emscripten_val::*;
use emscripten_functions::emscripten::{run_script, run_script_int};

fn main() {
    let a = Val::from_array(&[1, 2]);
    run_script(&format!(r#"
        console.log(Emval.toValue({}));
    "#, a.as_handle() as i32));

    a.call("push", argv![3]);
    run_script(&format!(r#"
        console.log(Emval.toValue({}));
    "#, a.as_handle() as i32));

    let handle = run_script_int("let n = new Number('123'); Emval.toHandle(n)");
    let number = Val::take_ownership(handle as EM_VAL);
    println!("{}", number.call("valueOf", &[]).as_i32());

    #[no_mangle]
    pub extern "C" fn event_handler(ev: EM_VAL) {
        let val = Val::take_ownership(ev);
        let target = val.get(&"target");
        target.set(&"textContent", &"Clicked");
    }

    let button = Val::take_ownership(run_script_int(r#"
        let button = document.createElement('BUTTON');
        button.addEventListener('click', (ev) => {
            _event_handler(Emval.toHandle(ev));
        });
        let body = document.getElementsByTagName('body')[0];
        body.appendChild(button);
        Emval.toHandle(button) 
    "#) as EM_VAL);
    button.set(&"textContent", &"click");
}