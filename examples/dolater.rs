pub fn dolater(input: &str, callback: Box<dyn FnOnce(String)>) {
    use emscripten_val::argv;
    use emscripten_val::Val;

    let js_fn_handle = Val::global("dolater");
    let mut callback_option = Some(callback);
    // dbg!("about to call dolater");
    js_fn_handle.invoke(argv![
        input,
        Val::from_fn1(move |content: &Val| {
            // callback is FnOnce, we turn it into FnMut using this Option.
            if let Some(callback) = callback_option.take() {
                let s = content.as_string();
                callback(s);
            };
            ().into()
        })
    ]);
}

fn main() {
    dolater(
        "rust string",
        Box::new(|s| {
            println!("got string: {}", s);
        }),
    );
}
