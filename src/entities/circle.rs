use crate::color;
use crate::position;
use crate::displacement;
use crate::shapeobject::ShapeObject;

use std::fmt;

pub struct Circle {
    pub name: String,
    pub radius: f64,
    pub color: color::ColorRGB,
    pub position: position::Position,
    pub displacement: displacement::Displacement
}

impl Circle {
    pub fn new(name: String, 
        radius: f64, 
        color: color::ColorRGB, 
        pos: position::Position, 
        displacement: displacement::Displacement) -> Self {
        Circle {
            name: name,
            radius: radius,
            color: color,
            position: pos,
            displacement: displacement
        }
    }

    fn new_from(&self) -> Self {
        Circle {
            name: self.name.clone(),
            radius: self.radius,

            color: self.color,
            position: self.position,
            displacement: self.displacement
        }
    } 
}

impl Clone for Circle {
    fn clone(&self) -> Self {
        return Circle::new_from(self);
    }
}

impl ShapeObject for Circle {
    fn get_displacement(&self) -> displacement::Displacement {
        self.displacement
    }
    fn set_displacement(&mut self, new: displacement::Displacement) {
        self.displacement = new;
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle(name: {}, radius: {}, color: {}, position: {}, displacement: {})", 
        self.name, self.radius, self.color, self.position, self.displacement)
    }
}
