#include <iostream>
#include <sstream>
#include <memory>
#include <string>
#include <vector>
#include <cstdint>

#include "cpp/declare_blob.hpp"
#include "cpp/declare_dummy_struct.hpp"
#include "cpp/ptr.hpp"
#include "cpp/maybe_ptr.hpp"
#include "cpp/list.hpp"
#include "cpp/string_ptr.hpp"
#include "cpp/shared_list.hpp"
#include "cpp/types/bytes.hpp"

// print-sizes
#ifdef PRINT_SIZES
int main()
{
    std::cout << "LIB_RUBY_PARSER_PTR_SIZE = " << sizeof(PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_MAYBE_PTR_SIZE = " << sizeof(MAYBE_PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_LIST_SIZE = " << sizeof(NodeList_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_STRING_PTR_SIZE = " << sizeof(STRING_PTR_BLOB_DATA) << "\n";
    std::cout << "LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE = " << sizeof(SHARED_BYTE_LIST_BLOB_DATA) << "\n";

    std::cout << "LIB_RUBY_PARSER_BYTES_SIZE = " << sizeof(ByteList_BLOB_DATA) << "\n";
}
#endif
