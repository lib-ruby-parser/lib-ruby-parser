#[derive(Debug)]
pub struct StackState {
    name: &'static str,
    stack: usize
}

impl StackState {
    pub fn new(name: &'static str) -> Self {
        Self { name, stack: 0 }
    }

    pub fn clear(&mut self) {
        self.stack = 0
    }

    pub fn push(&mut self, value: bool) -> bool {
        let bit_value: usize = if value { 1 } else { 0 };
        self.stack = (self.stack << 1) | bit_value;

        value
    }

    pub fn pop(&mut self) -> bool {
        let bit_value = self.stack & 1;
        self.stack >>= 1;

        bit_value == 1
    }

    pub fn lexpop(&mut self) -> bool {
        self.stack = (self.stack >> 1) | (self.stack & 1);
        self.is_active()
    }

    pub fn is_active(&self) -> bool {
        self.stack & 1 > 0
    }

    pub fn is_empty(&self) -> bool {
        self.stack == 0
    }

    pub fn to_s(&self) -> String {
        format!("{:b} <= {}", self.stack, self.name)
    }

    pub fn inspect(&self) -> String {
        self.to_s()
    }
}
