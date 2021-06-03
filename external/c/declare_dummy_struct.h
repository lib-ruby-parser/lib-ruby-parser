#ifndef LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_DUMMY_STRUCT_H
#define LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_DUMMY_STRUCT_H

#include <stdint.h>

typedef uint8_t BYTE;

#define DECLARE_DUMMY_STRUCT(NAME, SIZE) \
    typedef struct                       \
    {                                    \
        BYTE bytes[SIZE];                \
    } NAME

#endif // LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_DUMMY_STRUCT_H
