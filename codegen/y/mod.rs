mod parser_y;
use parser_y::generate_parser_y;

pub(crate) fn codegen() {
    generate_parser_y();
}
