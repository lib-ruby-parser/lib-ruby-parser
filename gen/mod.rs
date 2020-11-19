#[cfg(feature = "lib-ruby-parser-nodes")]
mod nodes;
#[cfg(feature = "lib-ruby-parser-nodes")]
pub use nodes::generate_nodes;

#[cfg(feature = "rust-bison-skeleton")]
mod parser_y;
#[cfg(feature = "rust-bison-skeleton")]
pub use parser_y::generate_parser_y;

#[cfg(not(feature = "lib-ruby-parser-nodes"))]
pub fn generate_nodes() {}

#[cfg(not(feature = "rust-bison-skeleton"))]
pub fn generate_parser_y() {}
