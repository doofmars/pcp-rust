// First pattern example using a matcher and option value
fn matcher1_option_value(candidate: Option<i32>) -> Option<i32> {
    match candidate {
        None => None,
        // match if we have some value, then return the increment
        Some(i) => Some(i + 1),
    }
}

fn matcher2_if_let(candidate: &str) -> i8 {
    let result_value: Result<i8, _> = candidate.parse();
    // Return parsing result matching the pattern "let Ok(age) = age"
    // The inner let reassigns result_value to the parsed variable of type i8
    // otherwise 0 if parsing is not Ok
    return if let Ok(result_value) = result_value {
        result_value
    } else {
        0
    }
}

fn matcher3_while_let(mut stack: Vec<i32>) {
    // Pop the last element of the stack until it is empty
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn matcher4_multiple_ranges(candidate: i32) -> i32 {
    // Matcher with pattern for "or" and "range" (3-10)
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
    matcher1_option_value(None);
    matcher2_if_let("23");
    matcher3_while_let(vec![1, 2, 3]);
}

#[test]
fn test_filter_map_reduce() {
    assert_eq!(matcher1_option_value(None), None);
    assert_eq!(matcher1_option_value(Option::from(4)), Some(5));
    assert_eq!(matcher2_if_let("wrong"), 0);
    assert_eq!(matcher2_if_let("10"), 10);
    assert_eq!(matcher4_multiple_ranges(0), 0);
    assert_eq!(matcher4_multiple_ranges(1), 1);
    assert_eq!(matcher4_multiple_ranges(2), 1);
    assert_eq!(matcher4_multiple_ranges(3), 2);
    assert_eq!(matcher4_multiple_ranges(5), 2);
    assert_eq!(matcher4_multiple_ranges(10), 2);
    assert_eq!(matcher4_multiple_ranges(11), 0);
}
