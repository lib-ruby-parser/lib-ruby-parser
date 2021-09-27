#include "string_ptr.hpp"
#include <cstdlib>

namespace lib_ruby_parser
{
    StringPtr::StringPtr() : ptr(nullptr), size(0) {}
    StringPtr::StringPtr(uint8_t *ptr, size_t size) : ptr(ptr), size(size) {}

    StringPtr::~StringPtr()
    {
        if (ptr == nullptr)
        {
            return;
        }

        free(ptr);
        ptr = nullptr;
        size = 0;
    }

    StringPtr::StringPtr(StringPtr &&other)
    {
        ptr = other.ptr;
        size = other.size;

        other.ptr = nullptr;
        other.size = 0;
    }
    StringPtr &StringPtr::operator=(StringPtr &&other)
    {
        this->~StringPtr();
        ptr = other.ptr;
        size = other.size;

        other.ptr = nullptr;
        other.size = 0;

        return *this;
    }
}
