#ifndef LIB_RUBY_PARSER_CPP_BINDINGS_LIST_HPP
#define LIB_RUBY_PARSER_CPP_BINDINGS_LIST_HPP

namespace lib_ruby_parser
{
    // Generic list
    template <typename T>
    class List
    {
    public:
        T *ptr;
        size_t size;
        size_t capacity;

        List();
        List(T *ptr, size_t size);
        ~List();

        List(const List<T> &) = delete;
        List<T> &operator=(List<T> const &) = delete;

        List(List<T> &&other);
        List<T> &operator=(List<T> &&other);

        void push(T item);
        T pop();
        T remove(size_t index);
        void shrink_to_fit();
        void reserve(size_t additional);
    };
}

#endif // LIB_RUBY_PARSER_CPP_BINDINGS_LIST_HPP
