use crate::shape::Shape;
use crate::dimensions::TwoDimensional;
use crate::attributes::Round;

pub trait Circle {
    fn get_radius(&self) -> i32;

    fn get_diameter(&self) -> i32 {
        self.get_radius() * 2
    }
}

// Circles are shapes
impl<T: Circle> Shape for T {
    fn get_shape_name(&self) -> String {
        String::from("Circle")
    }
}

// All Round and 2D Shapes are circles
//impl <T: Shape + Round + TwoDimensional> Circle for T {}

