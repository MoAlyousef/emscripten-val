#include <emscripten/val.h>
#include <emscripten/bind.h>
#include <typeinfo>
#include <functional>

emscripten::internal::TYPEID EmvalType = &typeid(emscripten::val);

using event_listener_cb = void (*)(emscripten::val, void *);

EMSCRIPTEN_BINDINGS(MyBindings) {
    emscripten::class_<std::function<void(emscripten::val)>>("ListenerCallback")
        .constructor<>()
        .function("_internal_func_", &std::function<void(emscripten::val)>::operator());
};

static emscripten::val func_to_val(std::function<void(emscripten::val)> &&func) {
    return emscripten::val(func)["_internal_func_"].call<emscripten::val>(
        "bind", emscripten::val(func)
    );
}

// emscripten::internal::TYPEID FuncType = &typeid(std::function<void(emscripten::val)>);

extern "C" void rust_caller(emscripten::EM_VAL, void *data);

std::function<void(emscripten::val)> *get_std_fn(void *data) {
    puts("H2");
    return new std::function<void(emscripten::val)>([=](emscripten::val v) {
        puts("h1");
        // rust_caller(v.as_handle(), data);
    });
}

extern "C" emscripten::EM_VAL _emval_take_fn(void *data) {
    auto f = get_std_fn(data);
    auto v = emscripten::val(std::move(*f));
    // std::function<void(emscripten::val)> vals[] = {*f};
    // return emscripten::internal::_emval_take_value(FuncType, (const void *)vals);
    return v.release_ownership();
}