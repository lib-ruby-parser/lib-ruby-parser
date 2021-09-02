#ifndef LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H
#define LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H

#include <stdint.h>

#define DECLARE_BLOB_FOR(VALUE)                               \
    typedef struct VALUE##_BLOB                               \
    {                                                         \
        uint8_t data[sizeof(VALUE)];                          \
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
    inline VALUE UNPACK_##VALUE(VALUE##_BLOB blob)            \
    {                                                         \
        VALUE##_BLOB_UNION u = {.as_blob = blob};             \
        return u.as_value;                                    \
    }                                                         \
    inline VALUE##_BLOB PACK_##VALUE(VALUE value)             \
    {                                                         \
        VALUE##_BLOB_UNION u = {.as_value = value};           \
        return u.as_blob;                                     \
    }

#endif // LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H
