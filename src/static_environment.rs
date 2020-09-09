use std::collections::HashSet;

pub struct StaticEnvironment<'a> {
    variables: HashSet<&'a str>,
    stack: Vec<HashSet<&'a str>>,
}

const FORWARD_ARGS: &'static str = "FORWARD_ARGS";

impl<'a> StaticEnvironment<'a> {
    pub fn new() -> Self {
        Self { variables: HashSet::new(), stack: vec![] }
    }

    pub fn reset(&mut self) {
        self.variables.clear();
        self.stack.clear();
    }

    pub fn extend_static(&mut self) {
        let mut variables: HashSet<&'a str> = HashSet::new();
        std::mem::swap(&mut variables, &mut self.variables);
        self.stack.push(variables);
    }

    pub fn extend_dynamic(&mut self) {
        self.stack.push(self.variables.clone());
    }

    pub fn unextend(&mut self) {
        self.variables = self.stack.pop().unwrap();
    }

    pub fn declare(&mut self, name: &'a str) {
        self.variables.insert(name);
    }

    pub fn is_declared(&self, name: &'a str) -> bool {
        self.variables.get(name).is_some()
    }

    pub fn declare_forward_args(&mut self) {
        self.declare(FORWARD_ARGS);
    }

    pub fn declared_forward_args(&self) -> bool {
        self.is_declared(FORWARD_ARGS)
    }
}
