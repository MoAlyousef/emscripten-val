# emscripten-val

A Rust wrapper around the emscripten/val api.

## Usage
Add emscripten-val to your Cargo.toml:
```toml
[dependencies]
emscripten-val = "0.1.4"
```

Then you can import and use the Val wrapper and its associated methods:
```rust
use emscripten_val::*;

fn main() {
    let a = Val::from_array(&[1, 2]);
    a.call("push", argv![3]);
    let console = Val::global("console");
    console.call("log", argv![a]);
}
```

```rust
use emscripten_val::*;

fn main() {
    let console = Val::global("console");
    let document = Val::global("document");
    let elem = document.call("createElement", argv!["BUTTON"]);
    elem.set(&"textContent", &"Click");
    let body = document.call("getElementsByTagName", argv!["body"]).get(&0);
    elem.call(
        "addEventListener",
        argv![
            "click",
            Val::from_fn(move |ev| {
                console.call("clear", &[]);
                println!("client x: {}", ev.get(&"clientX").as_i32());
                println!("hello from Rust");
            })
        ],
    );
    body.call("appendChild", argv![elem]);
}
```

```rust
use emscripten_val::*;

fn main() {
    #[allow(non_snake_case)]
    let mut AudioContext = Val::global("AudioContext");
    if !AudioContext.as_bool() {
        println!("No global AudioContext, trying webkitAudioContext");
        AudioContext = Val::global("webkitAudioContext");
    }

    println!("Got an AudioContext");
    let context = AudioContext.new(&[]);
    let oscillator = context.call("createOscillator", &[]);

    println!("Configuring oscillator");
    oscillator.set(&"type", &"triangle");
    oscillator.get(&"frequency").set(&"value", &261.63); // Middle C

    println!("Playing");
    oscillator.call("connect", argv![context.get(&"destination")]);
    oscillator.call("start", argv![0]);

    println!("All done!");
}
```

This crate can also be used to complement the emscripten-functions crate:
```rust
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
```

## Building
To build, you need:
- emsdk
- wasm32-unknown-emscripten target.

The emsdk can be installed by following the instructions [here](https://emscripten.org/docs/getting_started/downloads.html).

To get the rust target:
```bash
rustup target add wasm32-unknown-emscripten
```

Running the build, you only need to pass the target to cargo:
```
cargo build --target=wasm32-unknown-emscripten
```

## Passing flags to Emscripten
The most convenient way to pass extra flags to the emscripten toolchain is via a .cargo/config.toml file:
```toml
[target.wasm32-unknown-emscripten]
rustflags = ["-Clink-args=-sASYNCIFY=1 -sALLOW_MEMORY_GROWTH -sOFFSCREENCANVAS_SUPPORT=1"]
```

## Deployment
Building a program with an entry (main) with the emscripten toolchain generates a .wasm binary and javascript glue code. Both files need to be deployed together. Then you only need to import the javascript glue code into your html:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <script src="./dom.js"></script>
</body>
</html>
```
