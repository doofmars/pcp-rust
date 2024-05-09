
pub trait TwoDimensional {
    fn get_area(&self) -> i32;

    fn get_circumference(&self) -> i32;
}

// 3D
pub trait ThreeDimensional {
    fn get_volume(&self) -> i32;
}
