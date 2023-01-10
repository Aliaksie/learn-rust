pub fn luhn(cc_number: &str) -> bool {
    if cc_number.trim().len() < 2 {
        return false;
    }

    let digits: Option<Vec<u32>> = cc_number
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10)) // dont wory
        .collect();

    match digits {
        Some(digits) => {
            let sum = digits
                .iter()
                .rev()
                .enumerate()
                .map(|(i, d)| if i % 2 != 0 { d + d } else { *d })
                .map(|d| if d > 9 { d - 9 } else { d })
                .sum::<u32>();

            sum % 10 == 0
        }
        None => false,
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}