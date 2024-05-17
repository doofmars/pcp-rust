fn main() {
    let result = filter_map_reduce(
        &["Tim", "Kathy", "Tamara", "Bob"]
    );

    println!("Result: {}", result);
}

// Here we could use the type [&str; 4] for list instead to fix to exactly 4 values in the list
fn filter_map_reduce(list: &[&str]) -> String {
    list.iter()
        .filter(|e| e.starts_with("T"))
        .map(|e| e.to_uppercase())
        .collect::<Vec<String>>().join(" ")
}

#[test]
fn test_filter_map_reduce() {
    assert_eq!(
        filter_map_reduce(&["Tim", "Dong", "Tom", "Dang"]),
        "TIM TOM"
    )
}
