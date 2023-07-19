#[derive(Clone)]
pub struct InputFile {
    pub filepath: String,
    pub code: Vec<u8>,
}

impl InputFile {
    fn eval(code: Vec<u8>) -> Self {
        Self {
            filepath: String::from("(eval)"),
            code,
        }
    }
}

pub struct InputFiles {
    pub files: Vec<InputFile>,
}

impl InputFiles {
    pub fn new_eval(code: Vec<u8>) -> Self {
        Self {
            files: vec![InputFile::eval(code)],
        }
    }

    pub fn new_pattern(pattern: &str) -> Self {
        let files: Vec<InputFile> = glob::glob(pattern)
            .expect("invalid glob pattern")
            .map(|f| f.unwrap().to_str().unwrap().to_string())
            .map(|filepath| InputFile {
                code: std::fs::read(&filepath).unwrap(),
                filepath,
            })
            .collect();

        Self { files }
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<InputFile> {
        self.files.into_iter()
    }

    pub fn repeat(&mut self, n: usize) -> Self {
        let desired_len = self.len() * n;
        let files = self
            .files
            .clone()
            .into_iter()
            .cycle()
            .take(desired_len)
            .collect();
        Self { files }
    }

    pub fn new(
        code_to_eval: &Option<String>,
        pattern: &Option<String>,
        repeat: &Option<usize>,
    ) -> Self {
        let repeat = repeat.to_owned().unwrap_or(1);

        if let Some(code_to_eval) = code_to_eval {
            Self::new_eval(code_to_eval.to_owned().into_bytes())
        } else if let Some(pattern) = pattern {
            Self::new_pattern(pattern)
        } else {
            eprintln!("Either code to eval or pattern must be provided");
            std::process::exit(1)
        }
        .repeat(repeat)
    }

    pub fn empty() -> Self {
        Self { files: vec![] }
    }
}
