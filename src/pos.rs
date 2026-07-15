#[cfg_attr(feature = "debug", derive(Debug))]
pub struct PositionDelimiter {
    pub line: usize,
    pub col: usize,
}

impl PositionDelimiter {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
