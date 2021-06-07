#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_IMPL_BLOB_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_IMPL_BLOB_H

#define IMPL_BLOB_PACK(VALUE)                                  \
    VALUE##_BLOB_DATA PACK(VALUE value)                        \
    {                                                          \
        VALUE##_BLOB_UNION u = {.as_value = std::move(value)}; \
        return u.as_blob;                                      \
    }

#define IMPL_BLOB_UNPACK(VALUE)                   \
    VALUE UNPACK(VALUE##_BLOB_DATA blob)          \
    {                                             \
        VALUE##_BLOB_UNION u = {.as_blob = blob}; \
        return std::move(u.as_value);             \
    }

#define IMPL_BLOB(VALUE)  \
    IMPL_BLOB_PACK(VALUE) \
    IMPL_BLOB_UNPACK(VALUE)

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_IMPL_BLOB_H
