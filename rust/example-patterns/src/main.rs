// Patterns covered here are based on the places for patterns of the rust book
// https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html

// First pattern example using a pattern with match and option value
fn pattern1_option_value(candidate: Option<i32>) -> Option<i32> {
    match candidate {
        None => None,
        // match if we have some value, then return the increment
        Some(i) => Some(i + 1),
    }
}

// Pattern to evaluate parsing result
fn pattern2_if_let(candidate: &str) -> i8 {
    // Parse the string to type i8 wrapped as a Result.
    // The type used in the result determines the type to parse to.
    let result_value: Result<i8, _> = candidate.parse();
    // The inner let reassigns result_value to the parsed variable of type i8
    // otherwise 0 if parsing is not Ok
    return if let Ok(result_value) = result_value {
        // Variable can be returned as type is i8
        result_value
    } else {
        // Error case
        0
    };
}

// Usage of pattern in while loop
fn pattern3_while_let(mut stack: Vec<i32>) {
    // Pop the last element of the stack until it is empty
    // Some is required here as the return type of Pop is Option
    // Some will unwrap the value to the variable top if present
    // Once the end is reached the loop will break due to a type mismatch
    while let Some(top) = stack.pop() {
        // Top is now the resolved i32 value from the Vector
        println!("{:?}", top);
    }
}

// Matcher with pattern for "or" and "range" (3-10)
fn pattern5_multiple_ranges(candidate: i32) -> i32 {
    match candidate {
        1 | 2 => {
            println!("one or two");
            1
        }
        3..=10 => {
            println!("three through ten");
            2
        }
        _ => 0
    }
}

fn main() {
    pattern1_option_value(None);
    pattern2_if_let("23");
    pattern3_while_let(vec![1, 2, 3]);
    pattern5_multiple_ranges(4);
}

#[test]
fn test_filter_map_reduce() {
    assert_eq!(pattern1_option_value(None), None);
    assert_eq!(pattern1_option_value(Option::from(4)), Some(5));
    assert_eq!(pattern2_if_let("wrong"), 0);
    assert_eq!(pattern2_if_let("10"), 10);
    assert_eq!(pattern5_multiple_ranges(0), 0);
    assert_eq!(pattern5_multiple_ranges(1), 1);
    assert_eq!(pattern5_multiple_ranges(2), 1);
    assert_eq!(pattern5_multiple_ranges(3), 2);
    assert_eq!(pattern5_multiple_ranges(5), 2);
    assert_eq!(pattern5_multiple_ranges(10), 2);
    assert_eq!(pattern5_multiple_ranges(11), 0);
}
