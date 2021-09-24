#include <cstdlib>
#include <cstring>
#include <iostream>
#include "list.hpp"
#include "structs.hpp"
#include <execinfo.h>

namespace lib_ruby_parser
{
    // Generic list
    template <typename T>
    List<T>::List() : ptr(nullptr), size(0), capacity(0) {}

    template <typename T>
    List<T>::List(T *ptr, size_t size) : ptr(ptr), size(size), capacity(size) {}

    template <typename T>
    List<T>::~List()
    {
        if (ptr == nullptr)
            return;

        for (size_t i = 0; i < size; i++)
        {
            T *item = ptr + i;
            item->~T();
        }
        free(ptr);
    }

    template <typename T>
    List<T>::List(List<T> &&other)
    {
        this->ptr = other.ptr;
        this->size = other.size;
        this->capacity = other.capacity;

        other.ptr = nullptr;
        other.size = 0;
        other.capacity = 0;
    }
    template <typename T>
    List<T> &List<T>::operator=(List<T> &&other)
    {
        this->~List();
        this->ptr = other.ptr;
        this->size = other.size;
        this->capacity = other.capacity;

        other.ptr = nullptr;
        other.size = 0;
        other.capacity = 0;

        return *this;
    }

    template <typename T>
    void List<T>::push(T item)
    {
        T *old_ptr = ptr;

        if (size + 1 > capacity)
        {
            if (capacity == 0)
            {
                capacity += 1;
            }
            else
            {
                capacity *= 2;
            }
        }

        ptr = (T *)malloc(sizeof(T) * capacity);
        if (old_ptr != nullptr)
        {
            memcpy(ptr, old_ptr, sizeof(T) * size);
            free(old_ptr);
        }

        using blob_t = std::array<uint8_t, sizeof(T)>;

        union X
        {
            T value;
            blob_t bytes;
            ~X(){};
        };
        X x{std::move(item)};
        uint8_t *bytes = x.bytes.data();
        memcpy(ptr + size, bytes, sizeof(T));

        size++;
    }

    template <typename T>
    T List<T>::pop()
    {
        T item = std::move(ptr[size - 1]);
        size--;
        return std::move(item);
    }

    template <typename T>
    T List<T>::remove(size_t index)
    {
        T item = std::move(ptr[index]);
        memmove(ptr + index, ptr + index + 1, sizeof(T) * (size - index - 1));
        size--;
        return std::move(item);
    }

    template <typename T>
    void List<T>::shrink_to_fit()
    {
        // already shrinked
        if (size == capacity)
        {
            return;
        }

        T *prev_ptr = ptr;

        if (size == 0)
        {
            ptr = nullptr;
        }
        else
        {
            ptr = (T *)malloc(sizeof(T) * size);
        }

        if (prev_ptr != nullptr)
        {
            memcpy(ptr, prev_ptr, sizeof(T) * size);
            free(prev_ptr);
        }

        capacity = size;
    }

    template <typename T>
    void List<T>::reserve(size_t additional)
    {
        if (size + additional > capacity)
        {
            capacity = size + additional;
            T *old_ptr = ptr;
            if (capacity == 0)
            {
                ptr = nullptr;
            }
            else
            {
                ptr = (T *)malloc(sizeof(T) * capacity);
                memcpy(ptr, old_ptr, sizeof(T) * size);
            }
            if (old_ptr != nullptr)
            {
                free(old_ptr);
            }
        }
    }

    template class List<Byte>;
    template class List<Token>;
    template class List<Node>;
    template class List<Diagnostic>;
    template class List<Comment>;
    template class List<MagicComment>;
    template class List<SourceLine>;
}
