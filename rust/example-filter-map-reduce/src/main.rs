fn main() {
    let list = ["Tim", "Kathy", "Tamara", "Bob"];

    let result = list.iter()
        .filter(|e| e.starts_with("T"))
        .map(|e| e.to_uppercase())
        .collect::<Vec<String>>().join(" ");

    println!("Result: {}", result);
}
