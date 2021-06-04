#ifndef LIB_RUBY_PARSER_EXTERNAL_C_LOC_H
#define LIB_RUBY_PARSER_EXTERNAL_C_LOC_H

#include <stdint.h>

typedef struct
{
    uint64_t begin;
    uint64_t end;
} Loc;

#endif // LIB_RUBY_PARSER_EXTERNAL_C_LOC_H
