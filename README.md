# emscripten-val

A Rust wrapper around the emscripten/val api.

## Usage
Add emscripten-val to your Cargo.toml:
```toml
emscripten-val = "0.1.0"
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
    let a = Val::from_array(&[1, 2]);
    a.call("push", argv![3]);
    let console = Val::global("console");
    console.call("log", argv![a]);
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