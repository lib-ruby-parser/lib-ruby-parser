#[derive(Clone, Default)]
pub struct StackState {
    name: &'static str,
    stack: usize,
}

impl StackState {
    pub(crate) fn new(name: &'static str) -> Self {
        Self { name, stack: 0 }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack == 0
    }

    pub fn push(&mut self, bit: bool) {
        let bit_value = if bit { 1 } else { 0 };
        self.stack = (self.stack << 1) | bit_value
    }

    pub fn pop(&mut self) {
        self.stack >>= 1;
    }

    pub(crate) fn is_active(&self) -> bool {
        (self.stack & 1) == 1
    }
}

impl std::fmt::Debug for StackState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{:b} <= {}]", self.stack, self.name))
    }
}
