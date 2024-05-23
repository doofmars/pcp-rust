fn process_names(names: Vec<&str>) -> String {
    names.iter()
        .filter(|name| { 3 <= name.len() && name.len() <= 4 })
        .map(|e| e.to_uppercase())
        .collect::<Vec<String>>().join(" ")
}

fn main() {
    let result = process_names(vec!("yes", "no", "okay", "too long"));
    println!("Result is {}", result)
}

#[test]
fn test_conversion() {
    assert_eq!(
        process_names(vec!("Susanna", "Joe", "Lu", "Timmy", "Rafael", "Lisa")),
        "JOE LISA"
    )
}

