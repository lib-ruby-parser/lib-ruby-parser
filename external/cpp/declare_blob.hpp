#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_H

#include <cstdint>

typedef uint8_t BYTE;

#define DECLARE_BLOB_FOR(VALUE)                                \
    extern "C"                                                 \
    {                                                          \
        struct VALUE##_BLOB_DATA                               \
        {                                                      \
            BYTE data[sizeof(VALUE)];                          \
        };                                                     \
    }                                                          \
                                                               \
    extern "C"                                                 \
    {                                                          \
        union VALUE##_BLOB_UNION                               \
        {                                                      \
            typedef VALUE value_t;                             \
            typedef VALUE##_BLOB_DATA blob_t;                  \
                                                               \
            _Static_assert(sizeof(value_t) == sizeof(blob_t)); \
                                                               \
            value_t as_value;                                  \
            blob_t as_blob;                                    \
                                                               \
            ~VALUE##_BLOB_UNION() noexcept                     \
            {                                                  \
            }                                                  \
        };                                                     \
    }                                                          \
    VALUE UNPACK_##VALUE(VALUE##_BLOB_DATA blob)               \
    {                                                          \
        VALUE##_BLOB_UNION u = {.as_blob = blob};              \
        return std::move(u.as_value);                          \
    }                                                          \
                                                               \
    VALUE##_BLOB_DATA PACK_##VALUE(VALUE value)                \
    {                                                          \
        VALUE##_BLOB_UNION u = {.as_value = std::move(value)}; \
        return u.as_blob;                                      \
    }

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_H
