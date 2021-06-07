#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_HPP

#include <cstdint>

typedef uint8_t BYTE;

#define DECLARE_BLOB_STRUCTS(VALUE)                            \
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
    }

#define DECLARE_BLOB_UNPACK_FOR(VALUE) \
    VALUE UNPACK(VALUE##_BLOB blob);

#define DECLARE_BLOB_PACK_FOR(VALUE) \
    VALUE##_BLOB PACK(VALUE value);

#define DECLARE_BLOB_FOR(VALUE)    \
    DECLARE_BLOB_STRUCTS(VALUE)    \
    DECLARE_BLOB_UNPACK_FOR(VALUE) \
    DECLARE_BLOB_PACK_FOR(VALUE)

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_BLOB_HPP
