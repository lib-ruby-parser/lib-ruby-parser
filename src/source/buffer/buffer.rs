use std::collections::HashMap;
use std::cell::RefCell;

use crate::source::buffer::{reencode_string, BufferError};
use crate::source;

#[derive(Debug)]
pub struct Buffer {
    pub name: String,
    pub source: String,
    chars: Vec<char>,
    pub lines: Vec<String>,
    encoding: String,
    first_line: usize,

    line_begins: RefCell<Vec<(usize, usize)>>,
    line_for_position_cache: RefCell<HashMap<usize, usize>>,
    column_for_position_cache: RefCell<HashMap<usize, usize>>,
}

impl Buffer {
    fn new(name: &str, source: String, encoding: String) -> Buffer {
        let lines: Vec<String> = source.lines().map(|e| e.to_owned()).collect();
        let chars: Vec<char> = source.chars().collect();

        let mut line_begins: Vec<(usize, usize)> = vec![];
        let mut offset = 0;

        for (idx, line) in lines.iter().enumerate() {
            line_begins.push( (idx, offset) );
            offset += 1 + line.len();
        }

        Buffer {
            source,
            name: name.into(),
            lines,
            chars,
            encoding,
            first_line: 0,

            line_begins: RefCell::new(line_begins),
            line_for_position_cache: RefCell::new(HashMap::new()),
            column_for_position_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_from_source(name: &str, source: &str) -> Result<Buffer, BufferError> {
        let source: Vec<u8> = source.bytes().collect();
        let (source, encoding) = reencode_string(&source)?;
        Ok(Buffer::new(name, source, encoding))
    }

    pub fn new_from_file(name: &str, filepath: &str) -> Result<Buffer, BufferError> {
        let source = std::fs::read(filepath)?;
        let (source, encoding) = reencode_string(&source)?;

        Ok(Buffer::new(name, source, encoding))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> Option<&[char]> {
        if range.start < self.chars.len() && range.end < self.chars.len() {
            Some(&self.chars[range])
        } else {
            None
        }
    }

    pub fn decompose_position(&self, position: usize) -> Option<(usize, usize)> {
        self.line_for(position).map(&|(line_no, line_begin)| {
            (self.first_line + line_no, position - line_begin)
        })
    }

    pub fn line_for_position(&self, position: usize) -> Option<usize> {
        let mut cache = self.line_for_position_cache.borrow_mut();

        if let Some(value) = cache.get(&position) {
            return Some(*value)
        }

        if let Some((line_no, _)) = self.line_for(position) {
            let result = self.first_line + line_no;
            cache.insert(position, result);
            Some(result)
        } else {
            None
        }
    }

    pub fn column_for_position(&self, position: usize) -> Option<usize> {
        let mut cache = self.column_for_position_cache.borrow_mut();

        if let Some(value) = cache.get(&position) {
            return Some(*value)
        }

        if let Some((_, line_begin)) = self.line_for(position) {
            let result = position - line_begin;
            cache.insert(position, result);
            Some(result)
        } else {
            None
        }
    }

    pub fn source_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn source_line(&self, lineno: usize) -> &String {
        &self.lines[lineno - self.first_line]
    }

    pub fn line_range(&self, lineno: usize) -> Option<source::Range> {
        let line_begins = self.line_begins.borrow();

        let index = lineno - self.first_line + 1;
        if index <= 0 || index > line_begins.len() {
            None
        } else {
            let begin_pos = self.line_begin_for_line_no(index);
            let end_pos;

            if index == line_begins.len() {
                end_pos = Some(self.source.len());
            } else {
                end_pos = self.line_begin_for_line_no(index + 1);
            }

            match (begin_pos, end_pos) {
                (Some(begin_pos), Some(end_pos)) => {
                    Some(
                        source::Range::new(
                            self,
                            begin_pos,
                            end_pos
                        )
                    )
                },
                (_, _) => None
            }
        }
    }

    pub fn source_range(&self) -> source::Range {
        source::Range::new(self, 0, self.source.len())
    }

    pub fn last_line(&self) -> usize {
        self.lines.len() + self.first_line - 1
    }

    fn line_begin_for_line_no(&self, line_no: usize) -> Option<usize> {
        self.line_begins.borrow().iter().nth(line_no + 1).map(|(_line_no, line_begin)| *line_begin)
    }

    fn line_for(&self, position: usize) -> Option<(usize, usize)> {
        self.line_begins.borrow().iter().rev().find(|&(_line_no, line_begin)| {
            *line_begin <= position
        }).map(|e| *e)
    }
}
