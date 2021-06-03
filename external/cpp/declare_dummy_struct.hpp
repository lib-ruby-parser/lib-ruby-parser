#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_DUMMY_STRUCT_H
#define LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_DUMMY_STRUCT_H

#include <stdint.h>

typedef uint8_t BYTE;

#define DECLARE_DUMMY_STRUCT(NAME, SIZE) \
    struct NAME                          \
    {                                    \
        BYTE data[SIZE];                 \
    }

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_DECLARE_DUMMY_STRUCT_H
