#[derive(Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl Position {
    pub fn new() -> Self {
        Self { line: 1, col: 0 }
    }

    pub fn end_of_str(s: &str) -> Self {
        let mut line = 0;
        let mut pos = 0;
        for l in s.lines() {
            line += 1;
            pos = l.chars().count() as u32
        }
        Position { line, col: pos }
    }

    pub fn update_by(&mut self, s: &str) {
        let mut line = 0;
        let mut col = 0;
        let mut lines = s.lines().peekable();

        while let Some(l) = lines.next() {
            let is_last = lines.peek().is_none();
            if is_last {
                col = l.chars().count() as u32
            } else {
                line += 1;
            }
        }

        if line == 0 {
            self.col += col;
        } else {
            self.line += line;
            self.col = col;
        }
    }

    pub fn add_col(&mut self, col: u32) {
        self.col += col;
    }
}
