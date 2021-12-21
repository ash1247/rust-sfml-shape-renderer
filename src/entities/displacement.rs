use std::fmt;

pub struct Displacement {
    pub dx: f64,
    pub dy: f64
}

impl Displacement {
    pub fn new(dx: f64, dy: f64) -> Self {
        Displacement {
            dx: dx,
            dy: dy
        }
    }
}

impl Copy for Displacement {}

impl Clone for Displacement {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Display for Displacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Displacement(dx: {}, fy: {})", self.dx, self.dy)
    }
}