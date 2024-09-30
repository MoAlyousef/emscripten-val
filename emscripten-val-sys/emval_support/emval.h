#ifndef __EMVAL_H__
#define __EMVAL_H__

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _EM_VAL *EM_VAL;

typedef const void *TYPEID;

extern TYPEID BoolType;
extern TYPEID IntType;
extern TYPEID FloatType;
extern TYPEID VoidType;
extern TYPEID EmvalType;
extern TYPEID FuncType;

enum EM_METHOD_CALLER_KIND {
  FUNCTION = 0,
  CONSTRUCTOR = 1,
};

enum {
  _EMVAL_UNDEFINED = 2,
  _EMVAL_NULL = 4,
  _EMVAL_TRUE = 6,
  _EMVAL_FALSE = 8,
  _EMVAL_LAST_RESERVED_HANDLE = 8,
};

typedef struct _EM_DESTRUCTORS *EM_DESTRUCTORS;
typedef struct _EM_METHOD_CALLER *EM_METHOD_CALLER;
typedef double EM_GENERIC_WIRE_TYPE;
typedef const void *EM_VAR_ARGS;

void _emval_register_symbol(const char *);

void _emval_incref(EM_VAL value);
void _emval_decref(EM_VAL value);

void _emval_run_destructors(EM_DESTRUCTORS handle);

EM_VAL _emval_new_array(void);
EM_VAL _emval_new_array_from_memory_view(EM_VAL mv);
EM_VAL _emval_new_object(void);
EM_VAL _emval_new_cstring(const char *);
EM_VAL _emval_new_u8string(const char *);
EM_VAL _emval_new_u16string(const char16_t *);

EM_VAL _emval_take_value(TYPEID type, EM_VAR_ARGS argv);

EM_VAL _emval_get_global(const char *name);
EM_VAL _emval_get_module_property(const char *name);
EM_VAL _emval_get_property(EM_VAL object, EM_VAL key);
void _emval_set_property(EM_VAL object, EM_VAL key, EM_VAL value);
EM_GENERIC_WIRE_TYPE _emval_as(EM_VAL value, TYPEID returnType,
                               EM_DESTRUCTORS *destructors);
int64_t _emval_as_int64(EM_VAL value, TYPEID returnType);
uint64_t _emval_as_uint64(EM_VAL value, TYPEID returnType);

bool _emval_equals(EM_VAL first, EM_VAL second);
bool _emval_strictly_equals(EM_VAL first, EM_VAL second);
bool _emval_greater_than(EM_VAL first, EM_VAL second);
bool _emval_less_than(EM_VAL first, EM_VAL second);
bool _emval_not(EM_VAL object);

EM_GENERIC_WIRE_TYPE _emval_call(EM_METHOD_CALLER caller, EM_VAL func,
                                 EM_DESTRUCTORS *destructors, EM_VAR_ARGS argv);

// DO NOT call this more than once per signature. It will
// leak generated function objects!
EM_METHOD_CALLER
_emval_get_method_caller(unsigned argCount, // including return value
                         const TYPEID argTypes[], EM_METHOD_CALLER_KIND asCtor);
EM_GENERIC_WIRE_TYPE _emval_call_method(EM_METHOD_CALLER caller, EM_VAL handle,
                                        const char *methodName,
                                        EM_DESTRUCTORS *destructors,
                                        EM_VAR_ARGS argv);
EM_VAL _emval_typeof(EM_VAL value);
bool _emval_instanceof(EM_VAL object, EM_VAL constructor);
bool _emval_is_number(EM_VAL object);
bool _emval_is_string(EM_VAL object);
bool _emval_in(EM_VAL item, EM_VAL object);
bool _emval_delete(EM_VAL object, EM_VAL property);
[[noreturn]] bool _emval_throw(EM_VAL object);
EM_VAL _emval_await(EM_VAL promise);
EM_VAL _emval_iter_begin(EM_VAL iterable);
EM_VAL _emval_iter_next(EM_VAL iterator);

#if __cplusplus >= 202002L
void _emval_coro_suspend(EM_VAL promise, void *coro_ptr);
EM_VAL _emval_coro_make_promise(EM_VAL *resolve, EM_VAL *reject);
#endif

#ifdef __cplusplus
} // extern "C"
#endif

#endif