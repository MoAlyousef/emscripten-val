use emscripten_val::*;

fn main() {
    #[allow(non_snake_case)]
    let mut AudioContext = Val::global("AudioContext");
    if AudioContext.not() {
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
