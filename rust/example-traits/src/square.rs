use crate::circle::Circle;
use crate::shape::Shape;

pub trait  Square {
    fn get_width() -> i32;
}

// Squares are shapes
impl<T: Circle> Shape for T {
    fn get_shape_name(&self) -> String {
        String::from("Square")
    }
}
