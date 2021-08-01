/// Representation of any location in the given input
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Copy, Default)]
pub struct Loc {
    /// Begin of the `Loc` range
    pub begin: usize,
    /// End of the `Loc` range
    pub end: usize,
}

impl Loc {
    /// Constructs a new Loc struct
    pub fn new(begin: usize, end: usize) -> Loc {
        Self { begin, end }
    }

    /// Returns `begin` field of the `Loc`
    pub fn begin(&self) -> usize {
        self.begin
    }

    /// Returns `end` field of the `Loc`
    pub fn end(&self) -> usize {
        self.end
    }
}
