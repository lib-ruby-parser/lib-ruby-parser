#ifndef LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H
#define LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H

#include <stdint.h>

typedef uint8_t BYTE;

#define DECLARE_BLOB_FOR(VALUE)                                    \
    typedef struct                                                 \
    {                                                              \
        BYTE data[sizeof(VALUE)];                                  \
    } VALUE##_BLOB_DATA;                                           \
                                                                   \
    typedef union                                                  \
    {                                                              \
        _Static_assert(sizeof(VALUE) == sizeof(VALUE##_BLOB_DATA), \
                       "sizeof(VALUE) != sizeof(BLOB_DATA)");      \
                                                                   \
        VALUE as_value;                                            \
        VALUE##_BLOB_DATA as_blob;                                 \
    } VALUE##_BLOB_UNION;                                          \
                                                                   \
    VALUE UNPACK_##VALUE(VALUE##_BLOB_DATA blob)                   \
    {                                                              \
        VALUE##_BLOB_UNION u = {.as_blob = blob};                  \
        return u.as_value;                                         \
    }                                                              \
                                                                   \
    VALUE##_BLOB_DATA PACK_##VALUE(VALUE value)                    \
    {                                                              \
        VALUE##_BLOB_UNION u = {.as_value = value};                \
        return u.as_blob;                                          \
    }

#endif // LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_BLOB_H
