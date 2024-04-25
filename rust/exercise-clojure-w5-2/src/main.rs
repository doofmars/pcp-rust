fn main() {
    println!("Calculating diagonal of rectangle 35x50");

    let result = calculate_rectangle_diagonal(35, 50);

    println!("Result: {}", result)
}

fn calculate_rectangle_diagonal(length: i32, height: i32) -> f32 {
    ((length.pow(2) + height.pow(2)) as f32).sqrt()
}
