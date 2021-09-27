#ifndef LIB_RUBY_PARSER_CPP_BINDINGS_STRING_PTR_HPP
#define LIB_RUBY_PARSER_CPP_BINDINGS_STRING_PTR_HPP

#include <cstdint>
#include <cstddef>

namespace lib_ruby_parser
{
    class StringPtr
    {
    public:
        uint8_t *ptr;
        size_t size;

        StringPtr();
        StringPtr(uint8_t *ptr, size_t size);

        ~StringPtr();

        StringPtr(const StringPtr &) = delete;
        StringPtr &operator=(StringPtr const &) = delete;

        StringPtr(StringPtr &&other);
        StringPtr &operator=(StringPtr &&other);
    };

    using MaybeStringPtr = StringPtr;
}

#endif // LIB_RUBY_PARSER_CPP_BINDINGS_STRING_PTR_HPP
