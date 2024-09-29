use emscripten_val::*;

fn main() {
    unsafe {
        #[allow(non_snake_case)]
        let mut AudioContext = Val::global("AudioContext");
        if !AudioContext.as_bool() {
            println!("No global AudioContext, trying webkitAudioContext");
            AudioContext = Val::global("webkitAudioContext");
        }

        println!("Got an AudioContext");
        let context = AudioContext.new(argv![]);
        let oscillator = context.call("createOscillator", argv![]);

        println!("Configuring oscillator");
        oscillator.set(&"type", &"triangle");
        oscillator.get(&"frequency").set(&"value", &261.63); // Middle C

        println!("Playing");
        oscillator.call("connect", argv![context.get(&"destination")]);
        oscillator.call("start", argv![0]);

        println!("All done!");
    }
}
