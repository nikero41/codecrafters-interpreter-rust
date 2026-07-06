#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Location {
    pub line: u32,
    pub column: i32,
    pub offset: usize,
    prev_char: Option<char>,
}

impl Location {
    pub fn new(line: u32, column: i32, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
            prev_char: None,
        }
    }

    pub fn advance(&mut self, char: char) {
        if let Some(char) = self.prev_char {
            self.offset += char.len_utf8();

            self.column += 1;
            if char == '\n' {
                self.line += 1;
                self.column = -1;
            }
        }

        self.prev_char = Some(char);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebugInfo {
    pub location: Location,
}

impl DebugInfo {
    pub fn new(location: Location) -> Self {
        Self { location }
    }
}
