#ifndef LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_LIST_H
#define LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_LIST_H

#include <stdint.h>

#define DECLARE_LIST_OF(ITEM, LIST) \
    typedef struct                  \
    {                               \
        ITEM *ptr;                  \
        uint64_t len;               \
        uint64_t capacity;          \
    } LIST;

#endif // LIB_RUBY_PARSER_EXTERNAL_C_DECLARE_LIST_H
