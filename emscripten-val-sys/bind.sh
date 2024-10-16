bindgen emscripten-val-sys/emval_support/emval.h -o emscripten-val-sys/src/val.rs -- -xc++
bindgen emscripten-val-sys/emval_support/embind.h -o emscripten-val-sys/src/bind.rs -- -xc++