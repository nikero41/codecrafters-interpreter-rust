#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct Location {
    pub line: u32,
    pub column: i32,
    pub offset: usize,
}

impl Location {
    pub fn new(line: u32, column: i32, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

#[derive(Debug)]
pub struct LocationTracker {
    pub location: Location,
    prev_char: Option<char>,
}

impl LocationTracker {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            prev_char: None,
        }
    }

    pub fn advance(&mut self, char: char) {
        if let Some(char) = self.prev_char {
            self.location.offset += char.len_utf8();

            self.location.column += 1;
            if char == '\n' {
                self.location.line += 1;
                self.location.column = -1;
            }
        }

        self.prev_char = Some(char);
    }

    pub fn current(&self) -> Location {
        self.location
    }
}

impl Default for LocationTracker {
    fn default() -> Self {
        Self::new(Location::new(1, -1, 0))
    }
}
