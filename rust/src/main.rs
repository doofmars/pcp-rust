use std::collections::HashMap;
use rand::random;

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

fn main() {
    ternary_operator();
    makros();
    let args = &["hello", "world", "I", "am", "arguments"];
    varargs_with_array(args);
}

