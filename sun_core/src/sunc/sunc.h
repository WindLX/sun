// ExprotLib
typedef struct
{
    SunMetaHashMapC *metas;
    size_t meta_len;
    SunPointerHashMapC *values;
    size_t value_len;
} ExportLibC;

// HashMap<String, SunObject>
typedef struct
{
    const char *key;
    SunObjectC *object;
} SunMetaHashMapC;

// HashMap<String, SunPointer>
typedef struct
{
    const char *key;
    SunPointerC *pointer;
} SunPointerHashMapC;

// SunObject
typedef struct
{
    SunMetaC *meta;
} SunObjectC;

// SunMeta
typedef struct
{
    const char *name;
    MethodsHashMapC *methods;
    size_t method_len;
} SunMetaC;

// HashMap<String, Function>
typedef struct
{
    const char *key;
    FunctionC *method;
} MethodsHashMapC;

// Function的枚举
typedef enum
{
    RUST_FUNCTION,
    SYS_FUNCTION,
} FunctionType;

// Function
typedef struct
{
    FunctionType type;
    void *data;
} FunctionC;

// SunValue 的枚举
typedef enum
{
    SUN_NIL,
    SUN_BOOLEAN,
    SUN_NUMBER,
    SUN_STRING,
    SUN_TABLE,
    SUN_FUNCTION,
    SUN_CLASS,
} SunValueType;

// SunValue
typedef struct
{
    SunValueType type;
    union
    {
        bool boolean;
        double number;
        char *string;
        TableC *table;
        FunctionC *function;
        ClassC *_class;
    } data;
} SunValueC;

// SunPointer
typedef struct
{
    SunValueC *data;
} SunPointerC;

// Table
typedef struct
{
    SunPointerC *array;
    size_t array_len;
    SunPointerHashMapC *dict;
    size_t dict_len;
} TableC;

// Class
typedef struct
{
    const char *name;
    SunPointerHashMapC *attributes;
} ClassC;
