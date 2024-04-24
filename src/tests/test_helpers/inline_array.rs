#[derive(Clone, Copy)]
pub(crate) struct InlineArray<const MAX: usize, T: Copy> {
    pub(crate) len: usize,
    items: [Option<T>; MAX],
}

impl<const MAX: usize, T: Copy + core::fmt::Debug> core::fmt::Debug for InlineArray<MAX, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "InlineArray {{ len: {}, items: ", self.len)?;
        f.debug_list().entries(self.iter()).finish()?;
        write!(f, " }}")?;
        Ok(())
    }
}

impl<const MAX: usize, T: Copy> InlineArray<MAX, T> {
    pub(crate) fn new() -> Self {
        Self {
            len: 0,
            items: [None; MAX],
        }
    }

    pub(crate) fn push(&mut self, item: T) {
        assert!(
            MAX - self.len > 1,
            "can't push, MAX is {}, len is {}",
            MAX,
            self.len
        );
        self.items[self.len] = Some(item);
        self.len += 1;
    }

    pub(crate) fn iter(self) -> impl Iterator<Item = T> {
        InlineArrayIterator {
            array: self,
            pos: 0,
        }
    }
}

pub(crate) struct InlineArrayIterator<const MAX: usize, T: Copy> {
    array: InlineArray<MAX, T>,
    pos: usize,
}

impl<const MAX: usize, T: Copy> Iterator for InlineArrayIterator<MAX, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.array.len {
            return None;
        }
        let item = self.array.items[self.pos];
        self.pos += 1;
        item
    }
}
