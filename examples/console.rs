use emscripten_val::*;

fn main() {
    let a = Val::from_array(&[1, 2]);
    a.call("push", argv![3]);
    let console = Val::global("console");
    console.call("log", argv![a.clone()]);
    let vec: Vec<i32> = a.to_vec();
    println!("{:?}", vec);
}
