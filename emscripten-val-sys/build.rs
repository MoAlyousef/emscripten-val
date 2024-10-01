fn main() {
    println!("cargo:rerun-if-changed=emval_support/emval.h");
    println!("cargo:rerun-if-changed=emval_support/emval_support.cpp");
    let target = std::env::var("TARGET").unwrap();
    if target.contains("emscripten") {
        let host = std::env::var("HOST").unwrap();
        cc::Build::new()
            .file("emval_support/emval_support.cpp")
            .cpp(true)
            .compiler(if host.contains("windows") { "em++.bat" } else { "em++" })
            .compile("emval_support");
        println!("cargo:rustc-link-lib=embind");
    }
}
