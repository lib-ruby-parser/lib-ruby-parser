use std::ffi::OsString;

#[derive(Debug)]
pub(crate) struct Repeater(usize);

impl Default for Repeater {
    fn default() -> Self {
        Self(1)
    }
}

impl From<OsString> for Repeater {
    fn from(value: OsString) -> Self {
        let repeat = value
            .to_str()
            .expect("repeat value must be a UTF-8 string")
            .parse()
            .expect("repeat value must be a number");
        Self(repeat)
    }
}

impl Repeater {
    pub(crate) fn repeat<T>(&self, input: &mut Vec<T>)
    where
        T: Clone,
    {
        let desired_len = input.len() * self.0;
        let output = std::mem::take(input);
        *input = output.into_iter().cycle().take(desired_len).collect();
    }
}
