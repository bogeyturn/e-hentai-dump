pub struct StringArena {
    pub data: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct StrRef {
    start: usize,
    len: usize,
}

impl StringArena {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn finalize(&mut self) {
        self.data.shrink_to_fit();
    }

    pub fn add(&mut self, s: &str) -> StrRef {
        let start = self.data.len();
        self.data.extend_from_slice(s.as_bytes());
        StrRef {
            start,
            len: s.len(),
        }
    }

    pub fn get(&self, r: StrRef) -> &str {
        std::str::from_utf8(&self.data[r.start..r.start + r.len]).unwrap()
    }
}

pub struct Arena<T> {
    pub data: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
        }
    }

    pub fn add_slice(&mut self, items: Vec<T>) -> std::ops::Range<usize> {
        let start = self.data.len();
        self.data.extend(items);
        start..self.data.len()
    }

    pub fn get_range(&self, range: std::ops::Range<usize>) -> &[T] {
        &self.data[range]
    }

    pub fn finalize(&mut self) {
        self.data.shrink_to_fit();
    }
}
