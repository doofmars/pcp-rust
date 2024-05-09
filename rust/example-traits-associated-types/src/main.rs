// Generic Trait
trait TraitA<T> {
    fn trim_a(s: T) -> T;
}

impl TraitA<String> for Foo {
    fn trim_a(s: String) -> String {
       s.trim().to_string()
    }
}

// Trait with associated type
trait TraitB {
    type T;

    fn trim_b(s: Self::T) -> Self::T;
}

impl TraitB for Foo {
    type T = String;

    fn trim_b(s: String) -> String {
        s.trim().to_string()
    }
}

struct Foo;

fn main() {
    println!("Output A: {}", Foo::trim_a(String::from(" TEST ")));
    println!("Output B: {}", Foo::trim_b(String::from(" TEST ")));
}

#[test]
fn test_trait_a() {
    assert_eq!(
        Foo::trim_a(String::from(" test ")),
        "test"
    );
}

#[test]
fn test_trait_b() {
    assert_eq!(
        Foo::trim_b(String::from(" test ")),
        "test"
    );
}
