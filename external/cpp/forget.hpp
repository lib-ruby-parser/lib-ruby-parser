#ifndef LIB_RUBY_PARSER_EXTERNAL_CPP_FORGET_HPP
#define LIB_RUBY_PARSER_EXTERNAL_CPP_FORGET_HPP

#include <cstdint>
#include <iostream>

template <typename T>
void forget(T value)
{
    union U
    {
        T as_value;
        uint8_t raw[sizeof(T)];

        ~U()
        {
        }
    };

    U u = {.as_value = std::move(value)};
}

#endif // LIB_RUBY_PARSER_EXTERNAL_CPP_FORGET_HPP
