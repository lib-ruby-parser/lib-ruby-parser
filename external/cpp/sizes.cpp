#include <iostream>

#include "ptr.hpp"
#include "maybe_ptr.hpp"
#include "list.hpp"
#include "string_ptr.hpp"
#include "shared_byte_list.hpp"
#include "bytes.hpp"

int main()
{
    std::cout << "LIB_RUBY_PARSER_PTR_SIZE = " << sizeof(PTR) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAYBE_PTR_SIZE = " << sizeof(MAYBE_PTR) << "\n";
    std::cout << "LIB_RUBY_PARSER_LIST_SIZE = " << sizeof(LIST_OF_Node) << "\n";
    std::cout << "LIB_RUBY_PARSER_STRING_PTR_SIZE = " << sizeof(STRING_PTR) << "\n";
    std::cout << "LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE = " << sizeof(SHARED_BYTE_LIST) << "\n";

    std::cout << "LIB_RUBY_PARSER_BYTES_SIZE = " << sizeof(BYTES) << "\n";
}
