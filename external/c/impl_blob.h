#ifndef LIB_RUBY_PARSER_EXTERNAL_C_IMPL_BLOB_H
#define LIB_RUBY_PARSER_EXTERNAL_C_IMPL_BLOB_H

#define IMPL_BLOB(VALUE)                            \
    VALUE UNPACK_##VALUE(VALUE##_BLOB blob)         \
    {                                               \
        VALUE##_BLOB_UNION u = {.as_blob = blob};   \
        return u.as_value;                          \
    }                                               \
                                                    \
    VALUE##_BLOB PACK_##VALUE(VALUE value)          \
    {                                               \
        VALUE##_BLOB_UNION u = {.as_value = value}; \
        return u.as_blob;                           \
    }

#endif // LIB_RUBY_PARSER_EXTERNAL_C_IMPL_BLOB_H
