#include <emscripten.h>
#include <emscripten/bind.h>
#include <emscripten/em_macros.h>
#include <emscripten/val.h>
#include <string.h>
#include <string>

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
    
    if (v.isNull() || v.isUndefined()) {
        v.release_ownership();
        return strdup("");
    }
    
    if (v.isString()) {
        auto s = strdup(v.as<std::string>().c_str());
        v.release_ownership();
        return s;
    }
    
    auto str_val = v.call<val>("toString");
    auto s = strdup(str_val.as<std::string>().c_str());
    v.release_ownership();
    return s;
}

// If output_buffer is NULL, return the length of the string representation of
// object. Otherwise, write to output_buffer the string representation of
// object. If output_buffer is not large enough, undefined behavior occurs.
uint64_t _emval_as_bytes(EM_VAL object, char *output_buffer) {
    auto v = val::take_ownership(object);
    if (v.isNull() || v.isUndefined()) {
        v.release_ownership();
        return 0; // Nothing to write
    }

    std::string buffer;
    uint64_t len = 0;

    if (v.isString()) {
        buffer = v.as<std::string>();
        len    = buffer.size();
        if (output_buffer) {
            memcpy(output_buffer, buffer.data(), len);
        }
    } else {
        auto str_val = v.call<val>("toString");
        buffer       = str_val.as<std::string>();
        len          = buffer.size();
        if (output_buffer) {
            memcpy(output_buffer, buffer.data(), len);
        }
    }

    v.release_ownership();
    return len;
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

EM_JS(EM_VAL, _emval_take_value,
      (emscripten::internal::TYPEID type,
       emscripten::internal::EM_VAR_ARGS argv), {
  try {
    var t = registeredTypes[type];
    if (!t) {
      return Emval.toHandle(HEAPF64[(argv>>3)]);
    }
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
      if (typeof jsValue === 'number') out = +jsValue;
      else if (typeof jsValue === 'boolean') out = jsValue ? 1 : 0;
      else if (jsValue == null) out = 0;
      else {
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

int64_t _emval_as_int64(EM_VAL value, emscripten::internal::TYPEID returnType) {
    auto v = val::take_ownership(value);
    int64_t result = 0;
    
    if (returnType == internal::TypeID<int64_t>::get()) {
        result = v.as<int64_t>();
    } else if (returnType == internal::TypeID<long>::get()) {
        result = static_cast<int64_t>(v.as<long>());
    } else {
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

EM_JS(EM_VAL, _emval_call_method_raw,
      (EM_VAL object,
       const char* methodName,
       EM_VAL* argv,
       int argc), {
  try {
    var obj = Emval.toValue(object);
    var name = UTF8ToString(methodName);
    var fn = obj[name];
    var args = new Array(argc);
    for (var i = 0; i < argc; i++) {
      args[i] = Emval.toValue(HEAPU32[(argv>>2) + i]);
    }
    var ret = fn.apply(obj, args);
    return Emval.toHandle(ret);
  } catch (e) {
    console.error('_emval_call_method_raw error:', e);
    return 0;
  }
});

EM_JS(EM_VAL, _emval_construct_raw,
      (EM_VAL constructor,
       EM_VAL* argv,
       int argc), {
  try {
    var C = Emval.toValue(constructor);
    var args = new Array(argc);
    for (var i = 0; i < argc; i++) {
      args[i] = Emval.toValue(HEAPU32[(argv>>2) + i]);
    }
    var ret = new (Function.prototype.bind.apply(C, [null].concat(args)))();
    return Emval.toHandle(ret);
  } catch (e) {
    console.error('_emval_construct_raw error:', e);
    return 0;
  }
});

EM_JS(EM_VAL, _emval_call_function_raw,
      (EM_VAL fn,
       EM_VAL* argv,
       int argc), {
  try {
    var fn = Emval.toValue(fn);
    var args = new Array(argc);
    for (var i = 0; i < argc; i++) {
      args[i] = Emval.toValue(HEAPU32[(argv>>2) + i]);
    }
    var ret = fn.apply(undefined, args);
    return Emval.toHandle(ret);
  } catch (e) {
    console.error('_emval_call_function_raw error:', e);
    return 0;
  }
});
}