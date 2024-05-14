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

// implement Trait1 for all types
impl<T> Trait1 for T {}

// implement Trait3 for all that implement 1 and 2
impl<T: Trait1 + Trait2> Trait3 for T {}

// implement Trait2 for the example struct
impl Trait2 for ExampleStruct {}

struct ExampleStruct;

// the parameter subj has to have all three Traits implemented
fn print_all(subj: &(impl Trait1 + Trait2 + Trait3)) {
    println!("Trait 1: {}", subj.fn1());
    println!("Trait 2: {}", subj.fn2());
    println!("Trait 3: {}", subj.fn3());
}

fn main() {
    // the ExampleStruct has all three Traits implemented and can be passed to print_all
    // => Trait1 is implemented on all types
    // => Trait2 is implemented directly on the ExampleStruct
    // => Trait3 is implemented on all that implement Trait1 and 2, which the ExampleStruct does
    let example_struct = ExampleStruct {};
    print_all(&example_struct);

    // Trait1 is implemented on all types, including strings
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
