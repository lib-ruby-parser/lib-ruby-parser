mod gen;
use gen::{generate_nodes, generate_parser_y, generate_size_rs, link_with_external_structures};

fn main() {
    generate_parser_y();
    generate_nodes();

    generate_size_rs();
    link_with_external_structures();
}
