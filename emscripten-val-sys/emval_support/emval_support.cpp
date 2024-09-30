#include <emscripten.h>
#include <emscripten/bind.h>
#include <emscripten/em_macros.h>
#include <emscripten/val.h>
#include <string.h>
#include <string>

using namespace emscripten;

EMSCRIPTEN_BINDINGS(MyBindings) {
  class_<std::function<void(val)>>("ListenerCallback")
      .constructor<>()
      .function("handleEvent", &std::function<void(val)>::operator());
};

internal::TYPEID EmvalType = internal::TypeID<val>::get();
internal::TYPEID BoolType = internal::TypeID<bool>::get();
internal::TYPEID IntType = internal::TypeID<int>::get();
internal::TYPEID FloatType = internal::TypeID<float>::get();
internal::TYPEID VoidType = internal::TypeID<void>::get();
internal::TYPEID FuncType = internal::TypeID<std::function<void(val)>>::get();

extern "C" char *_emval_as_str(EM_VAL object) {
  internal::_emval_incref(object);
  auto v = val::take_ownership(object);
  auto s = strdup(v.as<std::string>().c_str());
  return s;
}

extern "C" void emscripten_val_rust_caller(EM_VAL em, void *data);

extern "C" void _emval_add_event_listener(EM_VAL em, const char *name,
                                          void *data) {
  auto v = val::take_ownership(em);
  auto func = val(std::function<void(val)>([=](val ev) {
    emscripten_val_rust_caller(ev.as_handle(), data);
  }));
  v.call<void>("addEventListener", std::string(name), func);
  v.release_ownership();
}

extern "C" EM_VAL _emval_take_fn(void *data) {
  auto func = val(std::function<void(val)>([=](val ev) {
    emscripten_val_rust_caller(ev.as_handle(), data);
  }));
  return func.release_ownership();
}