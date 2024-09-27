fn main() {
    println!("cargo:rerun-if-changed=emval_support/emval.h");
    println!("cargo:rerun-if-changed=emval_support/emval_support.cpp");
    println!("cargo:rerun-if-changed=emval_support/val_support.cpp");
    let emsdk = std::env::var("EMSDK").unwrap();
    let toolchain_file = std::path::PathBuf::from(emsdk)
        .join("upstream")
        .join("emscripten")
        .join("cmake")
        .join("Modules")
        .join("Platform")
        .join("Emscripten.cmake");
    let mut dst = cmake::Config::new("emval_support");
    dst.define("CMAKE_TOOLCHAIN_FILE", toolchain_file);
    dst.profile("Release");
    dst.build();
    // cc::Build::new()
    //     .file("src/emval_support.cpp")
    //     .cpp(true)
    //     .compile("emval_support");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=embind");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=emval_support");
}
