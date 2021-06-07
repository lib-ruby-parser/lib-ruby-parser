#ifndef LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H
#define LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H

#include <stdint.h>

typedef uint8_t BYTE;

#define DECLARE_BLOB_FOR(VALUE)                               \
    typedef struct                                            \
    {                                                         \
        BYTE data[sizeof(VALUE)];                             \
    } VALUE##_BLOB;                                           \
                                                              \
    typedef union                                             \
    {                                                         \
        _Static_assert(sizeof(VALUE) == sizeof(VALUE##_BLOB), \
                       "sizeof(VALUE) != sizeof(BLOB_DATA)"); \
                                                              \
        VALUE as_value;                                       \
        VALUE##_BLOB as_blob;                                 \
    } VALUE##_BLOB_UNION;                                     \
                                                              \
    VALUE UNPACK_##VALUE(VALUE##_BLOB blob);                  \
    VALUE##_BLOB PACK_##VALUE(VALUE value);

#endif // LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H
