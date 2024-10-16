#ifndef __EMBIND_H__
#define __EMBIND_H__

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _EM_VAL *EM_VAL;

typedef const void *TYPEID;

typedef int GenericEnumValue;

typedef void* GenericFunction;

void _embind_register_void(
    TYPEID voidType,
    const char* name);

void _embind_register_bool(
    TYPEID boolType,
    const char* name,
    bool trueValue,
    bool falseValue);

void _embind_register_integer(
    TYPEID integerType,
    const char* name,
    size_t size,
    int32_t minRange,
    uint32_t maxRange);

void _embind_register_bigint(
    TYPEID integerType,
    const char* name,
    size_t size,
    int64_t minRange,
    uint64_t maxRange);

void _embind_register_float(
    TYPEID floatType,
    const char* name,
    size_t size);

void _embind_register_std_string(
    TYPEID stringType,
    const char* name);

void _embind_register_std_wstring(
    TYPEID stringType,
    size_t charSize,
    const char* name);

void _embind_register_emval(
    TYPEID emvalType);

void _embind_register_memory_view(
    TYPEID memoryViewType,
    unsigned typedArrayIndex,
    const char* name);

void _embind_register_function(
    const char* name,
    unsigned argCount,
    const TYPEID argTypes[],
    const char* signature,
    GenericFunction invoker,
    GenericFunction function,
    bool isAsync);

void _embind_register_value_array(
    TYPEID tupleType,
    const char* name,
    const char* constructorSignature,
    GenericFunction constructor,
    const char* destructorSignature,
    GenericFunction destructor);

void _embind_register_value_array_element(
    TYPEID tupleType,
    TYPEID getterReturnType,
    const char* getterSignature,
    GenericFunction getter,
    void* getterContext,
    TYPEID setterArgumentType,
    const char* setterSignature,
    GenericFunction setter,
    void* setterContext);

void _embind_finalize_value_array(TYPEID tupleType);

void _embind_register_value_object(
    TYPEID structType,
    const char* fieldName,
    const char* constructorSignature,
    GenericFunction constructor,
    const char* destructorSignature,
    GenericFunction destructor);

void _embind_register_value_object_field(
    TYPEID structType,
    const char* fieldName,
    TYPEID getterReturnType,
    const char* getterSignature,
    GenericFunction getter,
    void* getterContext,
    TYPEID setterArgumentType,
    const char* setterSignature,
    GenericFunction setter,
    void* setterContext);

void _embind_finalize_value_object(TYPEID structType);

void _embind_register_class(
    TYPEID classType,
    TYPEID pointerType,
    TYPEID constPointerType,
    TYPEID baseClassType,
    const char* getActualTypeSignature,
    GenericFunction getActualType,
    const char* upcastSignature,
    GenericFunction upcast,
    const char* downcastSignature,
    GenericFunction downcast,
    const char* className,
    const char* destructorSignature,
    GenericFunction destructor);

void _embind_register_class_constructor(
    TYPEID classType,
    unsigned argCount,
    const TYPEID argTypes[],
    const char* invokerSignature,
    GenericFunction invoker,
    GenericFunction constructor);

void _embind_register_class_function(
    TYPEID classType,
    const char* methodName,
    unsigned argCount,
    const TYPEID argTypes[],
    const char* invokerSignature,
    GenericFunction invoker,
    void* context,
    unsigned isPureVirtual,
    bool isAsync);

void _embind_register_class_property(
    TYPEID classType,
    const char* fieldName,
    TYPEID getterReturnType,
    const char* getterSignature,
    GenericFunction getter,
    void* getterContext,
    TYPEID setterArgumentType,
    const char* setterSignature,
    GenericFunction setter,
    void* setterContext);

void _embind_register_class_class_function(
    TYPEID classType,
    const char* methodName,
    unsigned argCount,
    const TYPEID argTypes[],
    const char* invokerSignature,
    GenericFunction invoker,
    GenericFunction method,
    bool isAsync);

void _embind_register_class_class_property(
    TYPEID classType,
    const char* fieldName,
    TYPEID fieldType,
    const void* fieldContext,
    const char* getterSignature,
    GenericFunction getter,
    const char* setterSignature,
    GenericFunction setter);

EM_VAL _embind_create_inheriting_constructor(
    const char* constructorName,
    TYPEID wrapperType,
    EM_VAL properties);

void _embind_register_enum(
    TYPEID enumType,
    const char* name,
    size_t size,
    bool isSigned);

void _embind_register_enum_value(
    TYPEID enumType,
    const char* valueName,
    GenericEnumValue value);

void _embind_register_constant(
    const char* name,
    TYPEID constantType,
    double value);

void _embind_register_optional(
    TYPEID optionalType,
    TYPEID type);

void _embind_register_user_type(
    TYPEID type,
    const char* typeName);

// Register an InitFunc in the global linked list of init functions.
void _embind_register_bindings(struct InitFunc* f);

#ifdef __cplusplus
}
#endif

#endif