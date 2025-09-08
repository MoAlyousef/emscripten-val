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
    auto s = strdup(v.as<std::string>().c_str());
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

// Replacement for the removed _emval_take_value function
EM_VAL _emval_take_value(emscripten::internal::TYPEID type, emscripten::internal::EM_VAR_ARGS argv) {
    // For primitive types, we can use the direct conversion approach
    // This is a simplified implementation - you may need to extend it based on your specific needs
    
    // Get the type information and create appropriate val object
    if (type == internal::TypeID<bool>::get()) {
        bool* ptr = static_cast<bool*>(const_cast<void*>(argv));
        return val(*ptr).release_ownership();
    } else if (type == internal::TypeID<int>::get()) {
        int* ptr = static_cast<int*>(const_cast<void*>(argv));
        return val(*ptr).release_ownership();
    } else if (type == internal::TypeID<float>::get()) {
        float* ptr = static_cast<float*>(const_cast<void*>(argv));
        return val(*ptr).release_ownership();
    } else if (type == internal::TypeID<double>::get()) {
        double* ptr = static_cast<double*>(const_cast<void*>(argv));
        return val(*ptr).release_ownership();
    } else if (type == internal::TypeID<std::string>::get()) {
        std::string* ptr = static_cast<std::string*>(const_cast<void*>(argv));
        return val(*ptr).release_ownership();
    }
    
    // For unknown types, return undefined
    return val::undefined().release_ownership();
}

// Replacement for the removed _emval_as function
emscripten::internal::EM_GENERIC_WIRE_TYPE _emval_as(EM_VAL value, emscripten::internal::TYPEID returnType, emscripten::internal::EM_DESTRUCTORS* destructors) {
    (void)destructors;
    auto v = val::take_ownership(value);
    
    // Convert based on the requested return type
    if (returnType == internal::TypeID<bool>::get()) {
        bool result = v.as<bool>();
        v.release_ownership(); // Release since we took ownership
        return static_cast<emscripten::internal::EM_GENERIC_WIRE_TYPE>(result ? 1.0 : 0.0);
    } else if (returnType == internal::TypeID<int>::get()) {
        int result = v.as<int>();
        v.release_ownership();
        return static_cast<emscripten::internal::EM_GENERIC_WIRE_TYPE>(result);
    } else if (returnType == internal::TypeID<float>::get()) {
        float result = v.as<float>();
        v.release_ownership();
        return static_cast<emscripten::internal::EM_GENERIC_WIRE_TYPE>(result);
    } else if (returnType == internal::TypeID<double>::get()) {
        double result = v.as<double>();
        v.release_ownership();
        return result; // Already the right type
    } else if (returnType == internal::TypeID<std::string>::get()) {
        // For strings, we need to handle this differently since it's not a primitive
        // Cast the pointer to uintptr_t first, then to double
        std::string result = v.as<std::string>();
        v.release_ownership();
        char* str_ptr = strdup(result.c_str());
        uintptr_t ptr_value = reinterpret_cast<uintptr_t>(str_ptr);
        return static_cast<emscripten::internal::EM_GENERIC_WIRE_TYPE>(ptr_value);
    }
    
    // For unknown types, return 0
    v.release_ownership();
    return 0.0;
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
