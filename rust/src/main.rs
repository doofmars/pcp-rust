use std::collections::HashMap;
use rand::random;

// How to define lambdas in rust
// They are called Closures and types can be inferred or defined.
// Refer to https://doc.rust-lang.org/rust-by-example/fn/closures.html#closures
// Different to java, we have a function type so what type of closure a function call requires,
// can be defined directly.
fn lambda_example() {
    let constant = 4;

    // Annotated function with function type definition on the variable itself
    // This is verbose but just to show the type (without using any IDE features)
    let closure_annotated = |i:i32| -> i32 { i + constant };
    let closure_inferred = |i| i + constant;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
}

// How to achieve a ternary operator, use if as alternative
fn ternary_operator() {
    let coin_toss: bool = random();
    let result = if coin_toss { "Head" } else { "Tail" };
    println!("Result is {}", result)
}

// Show the usage of the print makro
fn makros() {
    let geht = 42;
    let makros: HashMap<String, String> = HashMap::new();
    println!("Dieser {} {} {}, {:#?}", "aufruf", geht, "mit", makros);
}

// Workaround but everything hast to be string
fn varargs_with_array(args: &[&str]) {
    for arg in args {
        println!("{}", arg);
    }
}

// Example file to show general language features of rust
fn main() {
    lambda_example();
    ternary_operator();
    makros();
    let args = &["hello", "world", "I", "am", "arguments"];
    varargs_with_array(args);
}

#[test]
fn test_smoke() {
    main()
}
