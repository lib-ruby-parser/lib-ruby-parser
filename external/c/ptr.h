#ifndef LIB_RUBY_PARSER_EXTERNAL_C_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_C_PTR_H

#include <stddef.h>
#include <stdlib.h>
#include "declare_blob.h"

typedef int DUMMY_PTR_VALUE;
typedef void(DropPtr)(void *);

// Ptr<T>
typedef DUMMY_PTR_VALUE *PTR;
_Static_assert(sizeof(PTR) == 8, "wrong sizeof(PTR)");
DECLARE_BLOB_FOR(PTR);

PTR_BLOB_DATA lib_ruby_parser_containers_make_ptr_blob(void *ptr)
{
    return PACK_PTR(ptr);
}

void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB_DATA blob, DropPtr drop)
{
    PTR ptr = UNPACK_PTR(blob);
    if (ptr != NULL)
    {
        drop(ptr);
        free(ptr);
    }
}

void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB_DATA blob)
{
    return UNPACK_PTR(blob);
}

PTR_BLOB_DATA lib_ruby_parser_containers_null_ptr_blob()
{
    return PACK_PTR(NULL);
}

#endif // LIB_RUBY_PARSER_EXTERNAL_C_PTR_H
