#include <iostream>

#include "ptr.hpp"
#include "maybe_ptr.hpp"
#include "list.hpp"
#include "string_ptr.hpp"
#include "shared_byte_list.hpp"
#include "bytes.hpp"
#include "token.hpp"
#include "error_level.hpp"
#include "loc.hpp"
#include "comment.hpp"
#include "messages.hpp"
#include "maybe_loc.hpp"
#include "maybe_string_ptr.hpp"

int main()
{
    std::cout << "LIB_RUBY_PARSER_PTR_SIZE = " << sizeof(PTR) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAYBE_PTR_SIZE = " << sizeof(MAYBE_PTR) << "\n";
    std::cout << "LIB_RUBY_PARSER_LIST_SIZE = " << sizeof(LIST_OF_Node) << "\n";
    std::cout << "LIB_RUBY_PARSER_STRING_PTR_SIZE = " << sizeof(STRING_PTR) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAYBE_STRING_PTR_SIZE = " << sizeof(MAYBE_STRING_PTR) << "\n";
    std::cout << "LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE = " << sizeof(SHARED_BYTE_LIST) << "\n";

    std::cout << "LIB_RUBY_PARSER_BYTES_SIZE = " << sizeof(Bytes) << "\n";
    std::cout << "LIB_RUBY_PARSER_TOKEN_SIZE = " << sizeof(Token) << "\n";
    std::cout << "LIB_RUBY_PARSER_ERROR_LEVEL_SIZE = " << sizeof(ErrorLevel) << "\n";
    std::cout << "LIB_RUBY_PARSER_LOC_SIZE = " << sizeof(Loc) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAYBE_LOC_SIZE = " << sizeof(MaybeLoc) << "\n";
    std::cout << "LIB_RUBY_PARSER_COMMENT_TYPE_SIZE = " << sizeof(CommentType) << "\n";
    std::cout << "LIB_RUBY_PARSER_COMMENT_SIZE = " << sizeof(Comment) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SIZE = " << sizeof(MagicCommentKind) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAGIC_COMMENT_SIZE = " << sizeof(MagicComment) << "\n";
    std::cout << "LIB_RUBY_PARSER_DIAGNOSTIC_MESSAGE_SIZE = " << sizeof(DiagnosticMessage) << "\n";
}
