use crate::attributes::Round;
use crate::circle::Circle;
use crate::dimensions::TwoDimensional;
use crate::square::Square;

mod circle;
mod dimensions;
mod square;
mod shape;
mod attributes;

struct Test {}

impl Circle for Test {
    fn get_radius(&self) -> i32 {
        23
    }
}


fn main() {
    let test = Test {

    };

    test.get_radius();
}
