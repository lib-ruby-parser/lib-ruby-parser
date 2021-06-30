#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_NODE_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_NODE_HPP

#include "declare_dummy_struct.hpp"
#include "declare_blob.hpp"
#include "declare_list.hpp"

DECLARE_DUMMY_STRUCT(Node, 168);
DECLARE_BLOB_FOR(Node);

DECLARE_LIST_OF(Node_BLOB, LIST_OF_Node);
DECLARE_BLOB_FOR(LIST_OF_Node);
_Static_assert(sizeof(LIST_OF_Node) == 24, "sizeof(LIST_OF_Node) == 24");

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_TYPES_NODE_HPP
