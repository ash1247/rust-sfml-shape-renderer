use std::fmt;

pub struct Config {
    pub window_height: u32,
    pub window_width: u32,
    pub font_path: String,
}

impl Config {
    pub fn new_empty() -> Self {
        Config {
            window_height: 0,
            window_width: 0,
            font_path: String::new()
        }
    }
    pub fn new(h: u32, w: u32, font: String) -> Self {
        Config {
            window_height: h,
            window_width: w,
            font_path: font
        }
    }
}

impl Clone for Config{
    fn clone(&self) -> Self {
        Config {
            window_height: self.window_height,
            window_width: self.window_width,
            font_path: self.font_path.clone()
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(width: {}, height: {}, font_path: {})", self.window_width, self.window_height, self.font_path)
    }
}