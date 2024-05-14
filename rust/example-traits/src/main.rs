// trait to with default implementation of the get_val function
trait Foo {
    fn get_val() -> String {
        String::from("foo")
    }
}

// structs without any fields
struct Bar;
struct Baz;

// implement the trait "Foo" for the struct "Bar"
impl Foo for Bar {
    // override the get_val function
    fn get_val() -> String {
        String::from("bar")
    }
}

// because get_val has a default implementation, it does not need to be overriden
impl Foo for Baz {}

fn main() {
    println!("get_val overridden: {}", Bar::get_val());
    println!("get_val not overridden: {}", Baz::get_val());
}

#[test]
fn test() {
    assert_eq!(
        Bar::get_val(),
        "bar"
    );

    assert_eq!(
        Baz::get_val(),
        "foo"
    );
}
