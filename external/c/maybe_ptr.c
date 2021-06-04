#include <stddef.h>
#include <stdlib.h>
#include "maybe_ptr.h"
#include "impl_blob.h"

IMPL_BLOB(MAYBE_PTR);

MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_make_maybe_ptr_blob(void *ptr)
{
    return PACK_MAYBE_PTR(ptr);
}

void lib_ruby_parser_containers_free_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob, DropMaybePtr drop)
{
    MAYBE_PTR maybe_ptr = UNPACK_MAYBE_PTR(blob);
    if (maybe_ptr != NULL)
    {
        drop(maybe_ptr);
        free(maybe_ptr);
    }
}

void *lib_ruby_parser_containers_raw_ptr_from_maybe_ptr_blob(MAYBE_PTR_BLOB_DATA blob)
{
    return UNPACK_MAYBE_PTR(blob);
}

MAYBE_PTR_BLOB_DATA lib_ruby_parser_containers_null_maybe_ptr_blob()
{
    return PACK_MAYBE_PTR(NULL);
}
