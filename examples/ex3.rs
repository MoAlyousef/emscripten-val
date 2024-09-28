use emscripten_val::*;

fn main() {
    unsafe {
        #[allow(non_snake_case)]
        let Number = Val::global("Number");
        let num = Number.new(argv!["123"]);
        let val = num.call("toFixed", argv![]);
        println!("is string? {}", val.is_string());
        println!("{}", val.as_string());
        let val = num.call("valueOf", argv![]);
        println!("is number? {}", val.is_number());
        println!("{}", val.as_i32());
    }
}