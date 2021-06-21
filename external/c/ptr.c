#include <stddef.h>
#include <stdlib.h>
#include "ptr.h"
#include "impl_blob.h"

IMPL_BLOB(PTR);

PTR_BLOB lib_ruby_parser__internal__containers__ptr__make(void *ptr)
{
    return PACK_PTR(ptr);
}

void lib_ruby_parser__internal__containers__ptr__free(PTR_BLOB blob, DropPtr drop)
{
    PTR ptr = UNPACK_PTR(blob);
    if (ptr != NULL)
    {
        drop(ptr);
        free(ptr);
    }
}

void *lib_ruby_parser__internal__containers__ptr__get_raw(PTR_BLOB blob)
{
    return UNPACK_PTR(blob);
}

PTR_BLOB lib_ruby_parser__internal__containers__ptr__make_null()
{
    return PACK_PTR(NULL);
}
