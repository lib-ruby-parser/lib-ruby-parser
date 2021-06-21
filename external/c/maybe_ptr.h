#ifndef LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_PTR_H

#include <stddef.h>
#include <stdlib.h>
#include "declare_blob.h"

typedef int DUMMY_MAYBE_PTR_VALUE;
typedef void(DropMaybePtr)(void *);

// MaybePtr<T>
typedef DUMMY_MAYBE_PTR_VALUE *MAYBE_PTR;
_Static_assert(sizeof(MAYBE_PTR) == 8, "wrong sizeof(MAYBE_PTR)");
DECLARE_BLOB_FOR(MAYBE_PTR);

MAYBE_PTR_BLOB lib_ruby_parser__internal__containers___maybe_ptr__make(void *ptr);
void lib_ruby_parser__internal__containers___maybe_ptr__free(MAYBE_PTR_BLOB blob, DropMaybePtr drop);
void *lib_ruby_parser__internal__containers___maybe_ptr__get_raw(MAYBE_PTR_BLOB blob);
MAYBE_PTR_BLOB lib_ruby_parser__internal__containers___maybe_ptr__make_null();

#endif // LIB_RUBY_PARSER_EXTERNAL_C_MAYBE_PTR_H
