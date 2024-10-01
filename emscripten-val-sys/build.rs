fn main() {
    println!("cargo:rerun-if-changed=emval_support/emval.h");
    println!("cargo:rerun-if-changed=emval_support/emval_support.cpp");
    cc::Build::new()
        .file("emval_support/emval_support.cpp")
        .cpp(true)
        .compiler("em++")
        .compile("emval_support");
    println!("cargo:rustc-link-lib=embind");
}
