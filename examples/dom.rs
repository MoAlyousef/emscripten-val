use emscripten_val::*;

fn main() {
    let console = Val::global("console");
    let document = Val::global("document");
    let elem = document.call("createElement", argv!["BUTTON"]);
    elem.set(&"textContent", &"Click");
    println!("{}", elem.get(&"textContent").as_string());
    let body = document.call("getElementsByTagName", argv!["body"]).get(&0);
    elem.call(
        "addEventListener",
        argv![
            "click",
            Val::from_fn1(move |ev| -> Val {
                console.call("clear", &[]);
                println!("client x: {}", ev.get(&"clientX").as_::<i32>());
                println!("hello from Rust");
                ().into()
            })
        ],
    );
    body.call("appendChild", &[&elem]);
}
