#include "error_level.h"
#include "impl_blob.h"

IMPL_BLOB(ErrorLevel);

ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_warning()
{
    return PACK_ErrorLevel(WARNING);
}
ErrorLevel_BLOB lib_ruby_parser__internal__containers__error_level__make_error()
{
    return PACK_ErrorLevel(ERROR);
}
bool lib_ruby_parser__internal__containers__error_level__is_warning(ErrorLevel_BLOB blob)
{
    return UNPACK_ErrorLevel(blob) == WARNING;
}
bool lib_ruby_parser__internal__containers__error_level__is_error(ErrorLevel_BLOB blob)
{
    return UNPACK_ErrorLevel(blob) == ERROR;
}
