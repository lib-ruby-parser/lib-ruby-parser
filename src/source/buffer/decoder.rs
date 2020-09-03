use regex::Regex;
use encoding::Encoding;
use crate::source::buffer::BufferError;

lazy_static! {
    static ref FIRST_TWO_LINES_RE: Regex = Regex::new(r"\A(.*)\n?(.*\n)?").unwrap();

    static ref ENCODING_RE: Regex = Regex::new(r"(?x)
        [\s\#](en)?coding\s*[:=]\s*
        (
            # Special-case: there's a UTF8-MAC encoding.
            (?P<a>utf8-mac)
            |
            # Chew the suffix; it's there for emacs compat.
            (?P<b>[A-Za-z0-9_-]+?)(-unix|-dos|-mac)
            |
            (?P<c>[A-Za-z0-9_-]+)
        )
    ").unwrap();
}

fn recognize_encoding(source: &Vec<u8>) -> Option<encoding::EncodingRef> {
    if source.is_empty() {
        return None;
    }

    let mut lines = source.split(|byte| *byte == 10);
    let first_line = lines.next().unwrap_or(&[] as &[u8]);
    let second_line = lines.next().unwrap_or(&[] as &[u8]);

    let encoding_line: &[u8];

    if first_line.starts_with(r"\xef\xbb\xbf".as_bytes()) {
        return Some(encoding::all::UTF_8);
    } else if first_line.starts_with("#!".as_bytes()) {
        encoding_line = second_line;
    } else {
        encoding_line = first_line;
    }

    if !encoding_line.starts_with("#".as_bytes()) {
        return None;
    }

    let encoding_line = String::from(String::from_utf8_lossy(encoding_line));

    let captures = ENCODING_RE.captures(&encoding_line).unwrap();

    let capture = captures.name("a").or(captures.name("b")).or(captures.name("c")).map(|m| m.as_str());

    if let Some(encoding_name) = capture {
        find_encoding(encoding_name)
    } else {
        None
    }
}

fn find_encoding(encoding_name: &str) -> Option<encoding::EncodingRef> {
    println!("Searching for encoding {}", encoding_name);
    let name_to_compare: &str = &encoding_name.to_uppercase()[..];

    match name_to_compare {
        "UTF-8" => Some(encoding::all::UTF_8),
        "KOI8-R" => Some(encoding::all::KOI8_R),
        _ => {
            println!("Unsupported encoding {}", encoding_name);
            None
        }
    }
}

pub fn reencode_string(input: &Vec<u8>) -> Result<(String, String), BufferError> {
    let recognized_encoding = recognize_encoding(&input);
    let applied_encoding: String;

    let decoded_source: String;

    match recognized_encoding {
        Some(recognized_encoding) => {
            match recognized_encoding.decode(&input, encoding::DecoderTrap::Strict) {
                Ok(output) => {
                    applied_encoding = recognized_encoding.name().to_owned();
                    decoded_source = output;
                },
                Err(err) => {
                    return Err(BufferError::EncodingError(err.into_owned()));
                }
            }
        },
        None => {
            // try utf-8
            match encoding::all::UTF_8.decode(&input, encoding::DecoderTrap::Strict) {
                Ok(output) => {
                    applied_encoding = String::from("utf-8");
                    decoded_source = output
                },
                Err(_) => {
                    // Ignore decoding error, it's not utf-8
                    // and we don't know what's the encoding of input file
                    return Err(BufferError::UnrecognizedEncoding)
                }
            }
        }
    };

    Ok((decoded_source, applied_encoding))
}
