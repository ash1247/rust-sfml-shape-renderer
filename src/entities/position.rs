use std::fmt;

pub struct Position {
    pub x: f64,
    pub y: f64
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position {
            x: x,
            y: y
        }
    }
}

impl Copy for Position {}

impl Clone for Position {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position(x: {}, y: {})", self.x, self.y)
    }
}