#include "source_line.h"
#include "impl_blob.h"

IMPL_BLOB(SourceLine);
IMPL_BLOB(LIST_OF_SourceLine);

// API

SourceLine_BLOB lib_ruby_parser_source_line_new(uint64_t start, uint64_t end, bool ends_with_eof)
{
    SourceLine source_line = {.start = start, .end = end, .ends_with_eof = ends_with_eof};
    return PACK_SourceLine(source_line);
}

uint64_t lib_ruby_parser_source_line_get_start(SourceLine_BLOB blob)
{
    return UNPACK_SourceLine(blob).start;
}
uint64_t lib_ruby_parser_source_line_get_end(SourceLine_BLOB blob)
{
    return UNPACK_SourceLine(blob).end;
}
bool lib_ruby_parser_source_line_get_ends_with_eof(SourceLine_BLOB blob)
{
    return UNPACK_SourceLine(blob).ends_with_eof;
}

SourceLine_BLOB lib_ruby_parser_source_line_set_start(SourceLine_BLOB blob, uint64_t start)
{
    SourceLine source_line = UNPACK_SourceLine(blob);
    source_line.start = start;
    return PACK_SourceLine(source_line);
}
SourceLine_BLOB lib_ruby_parser_source_line_set_end(SourceLine_BLOB blob, uint64_t end)
{
    SourceLine source_line = UNPACK_SourceLine(blob);
    source_line.end = end;
    return PACK_SourceLine(source_line);
}
SourceLine_BLOB lib_ruby_parser_source_line_set_ends_with_eof(SourceLine_BLOB blob, bool ends_with_eof)
{
    SourceLine source_line = UNPACK_SourceLine(blob);
    source_line.ends_with_eof = ends_with_eof;
    return PACK_SourceLine(source_line);
}
