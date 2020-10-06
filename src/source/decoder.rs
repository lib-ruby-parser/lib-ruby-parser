use regex::Regex;
use std::fmt;
use std::error::Error;

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

#[derive(Debug)]
pub enum InputError {
    UnableToRecognizeEncoding,
    UnsupportdEncoding(String),
    UnknownEncoding,
    EncodingError(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InputError {}

fn recognize_encoding(source: &Vec<u8>) -> Result<String, InputError> {
    if source.is_empty() {
        return Err(InputError::UnableToRecognizeEncoding);
    }

    let mut lines = source.split(|byte| *byte == 10);
    let first_line = lines.next().unwrap_or(&[] as &[u8]);
    let second_line = lines.next().unwrap_or(&[] as &[u8]);

    let encoding_line: &[u8];

    if first_line.starts_with(r"\xef\xbb\xbf".as_bytes()) {
        return Ok("utf-8".to_owned());
    } else if first_line.starts_with("#!".as_bytes()) {
        encoding_line = second_line;
    } else {
        encoding_line = first_line;
    }

    if !encoding_line.starts_with("#".as_bytes()) {
        return Err(InputError::UnableToRecognizeEncoding);
    }

    let encoding_line = String::from(String::from_utf8_lossy(encoding_line));

    let captures = match ENCODING_RE.captures(&encoding_line) {
        Some(captures) => captures,
        None => return Err(InputError::UnableToRecognizeEncoding)
    };

    let enc = captures.name("a")
        .or(captures.name("b"))
        .or(captures.name("c"))
        .map(|m| m.as_str().to_owned());

    match enc {
        Some(enc) => Ok(enc),
        None => Err(InputError::UnableToRecognizeEncoding)
    }
}

fn find_encoding(enc: &str) -> Result<encoding::EncodingRef, InputError> {
    match &enc.to_uppercase()[..] {
        "UTF-8" => Ok(encoding::all::UTF_8),
        "KOI8-R" => Ok(encoding::all::KOI8_R),
        _ => {
            println!("Unsupported encoding {}", enc);
            Err(InputError::UnsupportdEncoding(enc.to_owned()))
        }
    }
}

fn decode(input: &Vec<u8>, enc: &str) -> Result<String, InputError> {
    match find_encoding(enc)?.decode(input, encoding::DecoderTrap::Ignore) {
        Ok(output) => Ok(output),
        Err(err) => Err(InputError::EncodingError(err.into_owned()))
    }
}

pub fn decode_input(input: &Vec<u8>, enc: Option<String>) -> Result<(String, String), InputError> {
    match enc {
        Some(enc) => return Ok(( decode(input, &enc)?, enc )),
        _ => {}
    }

    let enc = recognize_encoding(input).unwrap_or("utf-8".to_owned());
    Ok(( decode(input, &enc)?, enc ))
}
