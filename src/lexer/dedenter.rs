pub struct Dedenter {
    dedent_level: usize,
    at_line_begin: bool,
    indent_level: usize
}

const TAB_WIDTH: usize = 8;

impl Dedenter {
    pub fn new(dedent_level: usize) -> Self {
        Self { dedent_level, at_line_begin: true, indent_level: 0 }
    }

    pub fn dedent(&mut self, string: &mut str) {
        unimplemented!("Dedenter.dedent {} {} {} {} {}", self.dedent_level, self.at_line_begin, self.indent_level, TAB_WIDTH, string)
    }

    pub fn interrupt(&mut self) {
        self.at_line_begin = false
    }
}
