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