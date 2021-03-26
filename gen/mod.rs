#[cfg(feature = "lib-ruby-parser-nodes")]
mod nodes;
#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) use nodes::generate_nodes;

#[cfg(feature = "rust-bison-skeleton")]
mod parser_y;
#[cfg(feature = "rust-bison-skeleton")]
pub(crate) use parser_y::generate_parser_y;

#[cfg(not(feature = "lib-ruby-parser-nodes"))]
pub(crate) fn generate_nodes() {}

#[cfg(not(feature = "rust-bison-skeleton"))]
pub(crate) fn generate_parser_y() {}
