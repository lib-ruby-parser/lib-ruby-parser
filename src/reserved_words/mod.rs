mod reserved_word;
pub use reserved_word::ReservedWord;
mod list;
pub(crate) use list::RESERVED_WORDS;

pub fn reserved_word(tok: &[u8]) -> Option<&'static ReservedWord> {
    let bucket = RESERVED_WORDS.get(tok.len())?;
    let idx = bucket
        .binary_search_by(|e| e.name.as_bytes().cmp(tok))
        .ok()?;

    Some(&bucket[idx])
}
