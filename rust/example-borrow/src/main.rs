// Example file for borrowing and move semantik

// Show a simple reassign
fn simple_reassign() {
    let var1 = 1;
    let var2 = var1;

    println!("Die Werte sind: {} und {}", var1, var2);
}

// Show a simple reassign
fn simple_string_reassign() {
    let var1: &str = "Foo"; // var1 is a reference here
    // The following would not work, as string is stored directly in var1. No reference is used.
    // let var1: String = String::from("hello");
    let var2 = var1;

    println!("Die Werte sind: {} und {}", var1, var2);
}

fn string_with_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // Output of string calculated in reference
    println!("Die Länge von '{}' ist {}.", s1, len);
}

fn mutable_string() {
    let mut s = String::from("hello");
    change(&mut s);
    // Output of altered string in function
    println!("{}", s)
}

fn main() {
    simple_reassign();
    simple_string_reassign();
    string_with_reference();
    mutable_string();
}

/// Calculate the length of a string using a reference
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Append " world" to a mutable string reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[test]
fn test_smoke() {
    main()
}
