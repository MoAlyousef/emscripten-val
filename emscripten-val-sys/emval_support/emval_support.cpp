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

internal::TYPEID EmvalType = internal::TypeID<val>::get();
internal::TYPEID BoolType  = internal::TypeID<bool>::get();
internal::TYPEID IntType   = internal::TypeID<int>::get();
internal::TYPEID FloatType = internal::TypeID<float>::get();

extern "C" {
EM_VAL emscripten_val_rust_caller0(void *data);
EM_VAL emscripten_val_rust_caller1(EM_VAL em, void *data);
EM_VAL emscripten_val_rust_caller2(EM_VAL em0, EM_VAL em1, void *data);
EM_VAL
emscripten_val_rust_caller3(EM_VAL em0, EM_VAL em1, EM_VAL em2, void *data);
EM_VAL emscripten_val_rust_caller4(
    EM_VAL em0, EM_VAL em1, EM_VAL em2, EM_VAL em3, void *data
);

char *_emval_as_str(EM_VAL object) {
    internal::_emval_incref(object);
    auto v = val::take_ownership(object);
    auto s = strdup(v.as<std::string>().c_str());
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
