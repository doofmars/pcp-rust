trait Foo {
    fn get_val() -> String {
        String::from("foo")
    }
}

struct Bar {}
impl Foo for Bar {
    fn get_val() -> String {
        String::from("bar")
    }
}

fn main() {
    println!("Output: {}", Bar::get_val())
}

#[test]
fn test() {
    assert_eq!(
        Bar::get_val(),
        "bar"
    )
}
