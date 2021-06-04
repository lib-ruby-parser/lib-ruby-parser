#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_IMPL_BLOB_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_IMPL_BLOB_H

#define IMPL_BLOB(VALUE)                                       \
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

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_IMPL_BLOB_H
