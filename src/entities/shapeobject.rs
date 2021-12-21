use crate::displacement::Displacement;

pub trait ShapeObject {
    fn get_displacement(&self) -> Displacement;
    fn set_displacement(&mut self, new: Displacement);
}