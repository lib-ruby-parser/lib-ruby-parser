#include "error_level.hpp"
#include "impl_blob.hpp"

IMPL_BLOB(ErrorLevel);

extern "C"
{
    ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_warning()
    {
        return PACK(ErrorLevel::WARNING);
    }
    ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_error()
    {
        return PACK(ErrorLevel::ERROR);
    }
    bool lib_ruby_parser__internal__containers__error_level__is_warning(ErrorLevel_BLOB blob)
    {
        return UNPACK(blob) == ErrorLevel::WARNING;
    }
    bool lib_ruby_parser__internal__containers__error_level__is_error(ErrorLevel_BLOB blob)
    {
        return UNPACK(blob) == ErrorLevel::ERROR;
    }
}
