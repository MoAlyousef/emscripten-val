#include <emscripten/val.h>
#include <emscripten/bind.h>
#include <string>
#include <string.h>


emscripten::internal::TYPEID EmvalType = &typeid(emscripten::val);

extern "C" char *_emval_as_str(emscripten::EM_VAL object) {
    emscripten::internal::_emval_incref(object);
    auto v = emscripten::val::take_ownership(object);
    auto s = strdup(v.as<std::string>().c_str());
    return s;
}

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

extern "C" emscripten::EM_VAL _emval_take_fn(void *data) {
    puts("H3");
    auto v = func_to_val(std::function<void(emscripten::val)>([](emscripten::val) {
        puts("here");
        // (void)data;
    }));
    auto ev = v.release_ownership();
    emscripten::internal::_emval_incref(ev);
    return ev;
}