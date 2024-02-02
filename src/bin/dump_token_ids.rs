fn parse_lines<'a>(line1: &'a str, line3: &'a str) -> Option<(&'a str, &'a str, &'a str)> {
    let line1 = line1.trim();
    let line3 = line3.trim();

    let comment = line1.strip_prefix("/// ")?;
    let line3 = line3.strip_prefix("pub const ")?.strip_suffix(';')?;
    let (name, value) = line3.split_once(": i32 = ")?;

    Some((comment, name, value))
}

fn main() {
    let src = std::fs::read_to_string("src/parser/parse.rs").unwrap();

    let tokens = src
        .lines()
        .zip(src.lines().skip(2))
        .filter_map(|(line1, line3)| parse_lines(line1, line3))
        .collect::<Vec<_>>();

    let out = tokens
        .into_iter()
        .fold(String::new(), |buf, (comment, name, value)| {
            format!("{}\n\n{}\n{}\n{}", buf, comment, name, value)
        });

    std::fs::write("target/tokens", out).unwrap();
}
