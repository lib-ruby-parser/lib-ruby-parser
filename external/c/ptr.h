#ifndef LIB_RUBY_PARSER_EXTERNAL_C_PTR_H
#define LIB_RUBY_PARSER_EXTERNAL_C_PTR_H

#include "declare_blob.h"

typedef int DUMMY_PTR_VALUE;
typedef void(DropPtr)(void *);

// Ptr<T>
typedef DUMMY_PTR_VALUE *PTR;
_Static_assert(sizeof(PTR) == 8, "wrong sizeof(PTR)");
DECLARE_BLOB_FOR(PTR);

PTR_BLOB lib_ruby_parser__internal__containers__ptr__make(void *ptr);
void lib_ruby_parser__internal__containers__ptr__free(PTR_BLOB blob, DropPtr drop);
void *lib_ruby_parser__internal__containers__ptr__get_raw(PTR_BLOB blob);
PTR_BLOB lib_ruby_parser__internal__containers__ptr__make_null();

#endif // LIB_RUBY_PARSER_EXTERNAL_C_PTR_H
