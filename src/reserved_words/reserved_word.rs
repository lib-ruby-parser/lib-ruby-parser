/// Representation of a reserved word in Ruby
#[derive(Debug)]
pub struct ReservedWord {
    pub(crate) name: &'static str,
    pub(crate) id: i32,
    pub(crate) modifier_id: i32,
    pub(crate) state: i32,
}
