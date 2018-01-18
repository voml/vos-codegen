use super::*;


impl Default for Position {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            file: "".to_string(),
        }
    }
}

impl Debug for VosStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { VosStatement::Table(v) => { Debug::fmt(v, f) } }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.file.is_empty() {
            true => { write!(f, "\"<anonymous>:{}:{}\"", self.start, self.end) }
            false => { write!(f, "\"{}:{}:{}\"", self.file, self.start, self.end) }
        }
    }
}