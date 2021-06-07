#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_HPP

#include <cstdint>

typedef uint8_t BYTE;

#define DECLARE_BLOB_FOR(VALUE)                                \
    extern "C"                                                 \
    {                                                          \
        struct VALUE##_BLOB                                    \
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
            typedef VALUE##_BLOB blob_t;                       \
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
    VALUE UNPACK(VALUE##_BLOB blob);                           \
    VALUE##_BLOB PACK(VALUE value);

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_HPP
