mod reserved_word;
pub use reserved_word::ReservedWord;
mod list;
pub(crate) use list::RESERVED_WORDS;

/// Returns a `ReservedWord` for a given string slice.
///
/// Returns `None` if given word is not a reserved word in Ruby.
pub fn reserved_word(tok: &str) -> Option<&'static ReservedWord> {
    let bucket = RESERVED_WORDS.get(tok.len())?;
    let idx = bucket
        .binary_search_by(|e| e.name.as_bytes().cmp(tok.as_bytes()))
        .ok()?;

    Some(&bucket[idx])
}
