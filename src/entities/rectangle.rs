use crate::color;
use crate::position;
use crate::displacement;
use crate::shapeobject::ShapeObject;

use std::fmt;
use std::clone::Clone;

pub struct Rectangle {
    pub name: String,
    pub height: f64,
    pub width: f64,
    pub color: color::ColorRGB,
    pub position: position::Position,
    pub displacement: displacement::Displacement
}

impl Rectangle {
    pub fn new(name: String, 
        h: f64, w: f64, 
        color: color::ColorRGB, 
        pos: position::Position, 
        displacement: displacement::Displacement) -> Self {
        Rectangle {
            name: name,
            height: h,
            width: w,
            color: color,
            position: pos,
            displacement: displacement
        }
    }

    fn new_from(&self) -> Self {
        Rectangle {
            name: self.name.clone(),
            height: self.height,
            width: self.width,
            color: self.color,
            position: self.position,
            displacement: self.displacement
        }
    } 
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        return Rectangle::new_from(self);
    }
}

impl ShapeObject for Rectangle {
    fn get_displacement(&self) -> displacement::Displacement {
        self.displacement
    }
    fn set_displacement(&mut self, new: displacement::Displacement) {
        self.displacement = new;
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle(name: {}, height: {}, width: {}, color: {}, position: {}, displacement: {})", 
        self.name, self.height, self.width, self.color, self.position, self.displacement)
    }
}

