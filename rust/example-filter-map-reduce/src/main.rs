fn main() {
    let result = filter_map_reduce(
        ["Tim", "Kathy", "Tamara", "Bob"]
    );

    println!("Result: {}", result);
}

fn filter_map_reduce(list: [&str; 4]) -> String {
    list.iter()
        .filter(|e| e.starts_with("T"))
        .map(|e| e.to_uppercase())
        .collect::<Vec<String>>().join(" ")
}

#[test]
fn test_filter_map_reduce() {
    assert_eq!(
        filter_map_reduce(["Tim", "Dong", "Tom", "Dang"]),
        "TIM TOM"
    )
}
