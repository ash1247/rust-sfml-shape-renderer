use std::fmt;

pub struct ColorRGB {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl ColorRGB {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        ColorRGB {
            red: r,
            green: g,
            blue: b
        }
    }
    pub fn black() -> Self {
        ColorRGB {
            red: 0,
            green: 0,
            blue: 0
        }
    }
}

impl Copy for ColorRGB {}

impl Clone for ColorRGB {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Display for ColorRGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ColorRGB(red: {}, green: {}, green: {})", self.red, self.green, self.blue)
    }
}