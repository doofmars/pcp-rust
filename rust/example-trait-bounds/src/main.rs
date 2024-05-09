trait Trait1 {
    fn fn1(&self) -> String {
        String::from("val1")
    }
}

trait Trait2 {
    fn fn2(&self) -> String {
        String::from("val2")
    }
}

trait Trait3 {
    fn fn3(&self) -> String {
        String::from("val3")
    }
}

// implement Trait 1 for all types
impl<T> Trait1 for T {}

// implement Trait 3 for all that implement 1 and 2
impl<T: Trait1 + Trait2> Trait3 for T {}

// implement Trait 2 for the example struct
impl Trait2 for ExampleStruct {}

struct ExampleStruct {}

fn print_all(subj: &(impl Trait1 + Trait2 + Trait3)) {
    println!("Trait 1: {}", subj.fn1());
    println!("Trait 2: {}", subj.fn2());
    println!("Trait 3: {}", subj.fn3());
}

fn main() {
    let example_struct = ExampleStruct {};
    print_all(&example_struct);

    println!("Call on string: {}", "".fn1());
}

#[test]
fn test_example_struct() {
    let example_struct = ExampleStruct {};

    assert_eq!(
        example_struct.fn1(),
        "val1"
    );
    assert_eq!(
        example_struct.fn2(),
        "val2"
    );
    assert_eq!(
        example_struct.fn3(),
        "val3"
    );
}

#[test]
fn test_string_type() {
    assert_eq!(
        "".fn1(),
        "val1"
    )
}
