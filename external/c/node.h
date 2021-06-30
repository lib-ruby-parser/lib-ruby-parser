#ifndef LIB_RUBY_PARSER_EXTERNAL_C_NODE_H
#define LIB_RUBY_PARSER_EXTERNAL_C_NODE_H

#include "declare_dummy_struct.h"
#include "declare_blob.h"
#include "declare_list.h"

DECLARE_DUMMY_STRUCT(Node, 176);
DECLARE_BLOB_FOR(Node);

DECLARE_LIST_OF(Node_BLOB, LIST_OF_Node);
DECLARE_BLOB_FOR(LIST_OF_Node);
_Static_assert(sizeof(LIST_OF_Node) == 24, "sizeof(LIST_OF_Node) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_C_NODE_H
