use crate::Lexer;

impl Lexer {
    pub fn is_lvar_defined(&self, name: &Vec<u8>) -> bool {
        self.p.static_env.is_declared(name)
    }
}
