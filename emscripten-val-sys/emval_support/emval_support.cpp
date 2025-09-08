#include <emscripten.h>
#include <emscripten/bind.h>
#include <emscripten/em_macros.h>
#include <emscripten/val.h>
#include <string.h>
#include <string>
#include <cstdint>

using namespace emscripten;

EMSCRIPTEN_BINDINGS(MyBindings) {
    class_<std::function<val()>>("ListenerCallback0")
        .constructor<>()
        .function("_internal_func_", &std::function<val()>::operator());
    class_<std::function<val(val)>>("ListenerCallback1")
        .constructor<>()
        .function("_internal_func_", &std::function<val(val)>::operator());
    class_<std::function<val(val, val)>>("ListenerCallback2")
        .constructor<>()
        .function("_internal_func_", &std::function<val(val, val)>::operator());
    class_<std::function<val(val, val, val)>>("ListenerCallback3")
        .constructor<>()
        .function(
            "_internal_func_", &std::function<val(val, val, val)>::operator()
        );
    class_<std::function<val(val, val, val, val)>>("ListenerCallback4")
        .constructor<>()
        .function(
            "_internal_func_",
            &std::function<val(val, val, val, val)>::operator()
        );
};

template <typename... T>
static val func_to_val(std::function<val(T...)> &&func) {
    return val(func)["_internal_func_"].call<val>(
        "bind", emscripten::val(func)
    );
}


extern "C" {
EM_VAL emscripten_val_rust_caller0(void *data);
EM_VAL emscripten_val_rust_caller1(EM_VAL em, void *data);
EM_VAL emscripten_val_rust_caller2(EM_VAL em0, EM_VAL em1, void *data);
EM_VAL
emscripten_val_rust_caller3(EM_VAL em0, EM_VAL em1, EM_VAL em2, void *data);
EM_VAL emscripten_val_rust_caller4(
    EM_VAL em0, EM_VAL em1, EM_VAL em2, EM_VAL em3, void *data
);
internal::TYPEID EmvalType() { return internal::TypeID<val>::get(); }
char *_emval_as_str(EM_VAL object) {
    auto v = val::take_ownership(object);
    
    // Check if the value is null or undefined
    if (v.isNull() || v.isUndefined()) {
        v.release_ownership();
        return strdup(""); // Return empty string for null/undefined
    }
    
    // If it's already a string, use it directly
    if (v.isString()) {
        auto s = strdup(v.as<std::string>().c_str());
        v.release_ownership();
        return s;
    }
    
    // For other types, convert to string using JavaScript's toString
    auto str_val = v.call<val>("toString");
    auto s = strdup(str_val.as<std::string>().c_str());
    v.release_ownership();
    return s;
}

void _emval_add_event_listener(EM_VAL em, const char *name, void *data) {
    auto v    = val::take_ownership(em);
    auto func = func_to_val(std::function<val(val)>([=](val ev) -> val {
        return val::take_ownership(
            emscripten_val_rust_caller1(ev.as_handle(), data)
        );
    }));
    v.call<void>("addEventListener", std::string(name), func);
    v.release_ownership();
}

// Implementations that delegate to Embind's JS type registry so that
// user-registered types (classes, pointers, strings, etc.) are handled
// correctly. These use EM_JS to interact with the JS runtime.

extern "C" {
EM_JS(EM_VAL, _emval_take_value,
      (emscripten::internal::TYPEID type,
       emscripten::internal::EM_VAR_ARGS argv), {
  try {
    var t = registeredTypes[type];
    if (!t) {
      // As a last resort, treat as a double read. This covers simple numbers
      // but won't help complex/user types. Better to fail loudly in dev.
      // console.warn('_emval_take_value: unknown type id', type);
      return Emval.toHandle(HEAPF64[(argv>>3)]);
    }
    // Obtain the JS value directly using the registered type's reader
    var jsValue = t.readValueFromPointer(argv);
    return Emval.toHandle(jsValue);
  } catch (e) {
    console.error('_emval_take_value exception:', e);
    return 0;
  }
});

EM_JS(emscripten::internal::EM_GENERIC_WIRE_TYPE, _emval_as,
      (EM_VAL value,
       emscripten::internal::TYPEID returnType,
       emscripten::internal::EM_DESTRUCTORS* destructors), {
  try {
    var t = registeredTypes[returnType];
    var jsValue = Emval.toValue(value);
    var d = [];
    var out;
    if (t) {
      out = t.toWireType(d, jsValue);
    } else {
      // Fallback heuristics for primitives if type isn't known
      if (typeof jsValue === 'number') out = +jsValue;
      else if (typeof jsValue === 'boolean') out = jsValue ? 1 : 0;
      else if (jsValue == null) out = 0;
      else {
        // Unknown and non-primitive
        // console.warn('_emval_as: unknown return type id', returnType, 'for value', jsValue);
        out = 0;
      }
    }
    if (destructors) {
      HEAPU32[(destructors>>2)] = d.length ? Emval.toHandle(d) : 0;
    }
    return out;
  } catch (e) {
    console.error('_emval_as exception:', e);
    if (destructors) HEAPU32[(destructors>>2)] = 0;
    return 0;
  }
});
}

// Implementations for specialized int64/uint64 functions
int64_t _emval_as_int64(EM_VAL value, emscripten::internal::TYPEID returnType) {
    auto v = val::take_ownership(value);
    int64_t result = 0;
    
    if (returnType == internal::TypeID<int64_t>::get()) {
        result = v.as<int64_t>();
    } else if (returnType == internal::TypeID<long>::get()) {
        result = static_cast<int64_t>(v.as<long>());
    } else {
        // Fallback to regular int conversion
        result = static_cast<int64_t>(v.as<int>());
    }
    
    v.release_ownership();
    return result;
}

uint64_t _emval_as_uint64(EM_VAL value, emscripten::internal::TYPEID returnType) {
    auto v = val::take_ownership(value);
    uint64_t result = 0;
    
    if (returnType == internal::TypeID<uint64_t>::get()) {
        result = v.as<uint64_t>();
    } else if (returnType == internal::TypeID<unsigned long>::get()) {
        result = static_cast<uint64_t>(v.as<unsigned long>());
    } else {
        // Fallback to regular unsigned int conversion
        result = static_cast<uint64_t>(v.as<unsigned int>());
    }
    
    v.release_ownership();
    return result;
}

EM_VAL _emval_take_fn(unsigned char argcount, void *data) {
    val func;
    switch (argcount) {
    case 0:
        func = func_to_val(std::function<val()>([=]() -> val {
            return val::take_ownership(emscripten_val_rust_caller0(data));
        }));
        break;
    case 1:
        func = func_to_val(std::function<val(val)>([=](val ev) -> val {
            return val::take_ownership(
                emscripten_val_rust_caller1(ev.as_handle(), data)
            );
        }));
        break;
    case 2:
        func = func_to_val(
            std::function<val(val, val)>([=](val a0, val a1) -> val {
                return val::take_ownership(emscripten_val_rust_caller2(
                    a0.as_handle(), a1.as_handle(), data
                ));
            })
        );
        break;
    case 3:
        func = func_to_val(std::function<val(val, val, val)>(
            [=](val a0, val a1, val a2) -> val {
                return val::take_ownership(emscripten_val_rust_caller3(
                    a0.as_handle(), a1.as_handle(), a2.as_handle(), data
                ));
            }
        ));
        break;
    case 4:
        func = func_to_val(std::function<val(val, val, val, val)>(
            [=](val a0, val a1, val a2, val a3) -> val {
                return val::take_ownership(emscripten_val_rust_caller4(
                    a0.as_handle(),
                    a1.as_handle(),
                    a2.as_handle(),
                    a3.as_handle(),
                    data
                ));
            }
        ));
    default:
        func = func_to_val(std::function<val(val)>([=](val ev) -> val {
            return val::take_ownership(
                emscripten_val_rust_caller1(ev.as_handle(), data)
            );
        }));
    }
    return func.release_ownership();
}
}
