mod gen;
use gen::{generate_nodes, generate_parser_y};

fn main() {
    generate_parser_y();
    generate_nodes();
}
