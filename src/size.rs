#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(1, 1)
    }
}
