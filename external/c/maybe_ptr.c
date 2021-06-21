#include <stddef.h>
#include <stdlib.h>
#include "maybe_ptr.h"
#include "impl_blob.h"

IMPL_BLOB(MAYBE_PTR);

MAYBE_PTR_BLOB lib_ruby_parser__internal__containers___maybe_ptr__make(void *ptr)
{
    return PACK_MAYBE_PTR(ptr);
}

void lib_ruby_parser__internal__containers___maybe_ptr__free(MAYBE_PTR_BLOB blob, DropMaybePtr drop)
{
    MAYBE_PTR maybe_ptr = UNPACK_MAYBE_PTR(blob);
    if (maybe_ptr != NULL)
    {
        drop(maybe_ptr);
        free(maybe_ptr);
    }
}

void *lib_ruby_parser__internal__containers___maybe_ptr__get_raw(MAYBE_PTR_BLOB blob)
{
    return UNPACK_MAYBE_PTR(blob);
}

MAYBE_PTR_BLOB lib_ruby_parser__internal__containers___maybe_ptr__make_null()
{
    return PACK_MAYBE_PTR(NULL);
}
