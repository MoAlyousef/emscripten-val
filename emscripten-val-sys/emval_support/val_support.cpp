#include <emscripten.h>
#include <emscripten/bind.h>
#include <emscripten/val.h>
#include <string.h>
#include <string>

using namespace emscripten;

internal::TYPEID EmvalType = &typeid(val);

extern "C" char *_emval_as_str(EM_VAL object) {
  internal::_emval_incref(object);
  auto v = val::take_ownership(object);
  auto s = strdup(v.as<std::string>().c_str());
  return s;
}

EMSCRIPTEN_BINDINGS(MyBindings) {
  class_<std::function<void(val)>>("ListenerCallback")
      .constructor<>()
      .function("_internal_func_", &std::function<void(val)>::operator());
};

static val func_to_val(std::function<void(val)> &&func) {
  return val(func)["_internal_func_"].call<val>("bind", val(func));
}

extern "C" void rust_caller(EM_VAL em, void *data);

extern "C" void _emval_add_event_listener(EM_VAL em, const char *name,
                                          void *data) {
  auto v = val::take_ownership(em);
  v.call<void>("addEventListener", std::string(name), func_to_val([=](val ev) {
                 rust_caller(ev.release_ownership(), data);
               }));
  v.release_ownership();
}