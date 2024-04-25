use std::io::{self};
use std::io::Write;

// Rekursive Fibonacci-Berechnung mit Ein- und Ausgabe
// Implementieren Sie nun ein Prädikat io_fib/0 so, dass der Benutzer zuerst eine Zahl
// eingeben muss, und dann die dazugehörende Fibonacci-Zahl ausgegeben wird.
fn main() {
    print!("Gib eine Zahl ein: ");
    // Flush the stdout to ensure the prompt is printed the console
    io::stdout().flush().unwrap();
    // Read a line as string
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    // Trim and parse to int 32
    let n: i32 = input_line.trim().parse().expect("Input not an integer");
    let fib = fibonacci_recursive(n);
    print!("Die {}. Fibonacci-Zahl ist: {}", n, fib);
}

fn fibonacci_recursive(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

#[test]
fn test_fibonacci_recursive() {
    assert_eq!(0, fibonacci_recursive(0));
    assert_eq!(1, fibonacci_recursive(1));
    assert_eq!(1, fibonacci_recursive(2));
    assert_eq!(2, fibonacci_recursive(3));
    assert_eq!(3, fibonacci_recursive(4));
}
