#include <stddef.h>
#include <stdlib.h>
#include "ptr.h"
#include "impl_blob.h"

IMPL_BLOB(PTR);

PTR_BLOB lib_ruby_parser_containers_make_ptr_blob(void *ptr)
{
    return PACK_PTR(ptr);
}

void lib_ruby_parser_containers_free_ptr_blob(PTR_BLOB blob, DropPtr drop)
{
    PTR ptr = UNPACK_PTR(blob);
    if (ptr != NULL)
    {
        drop(ptr);
        free(ptr);
    }
}

void *lib_ruby_parser_containers_raw_ptr_from_ptr_blob(PTR_BLOB blob)
{
    return UNPACK_PTR(blob);
}

PTR_BLOB lib_ruby_parser_containers_null_ptr_blob()
{
    return PACK_PTR(NULL);
}
