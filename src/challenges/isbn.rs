use std::str::FromStr;

pub struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
pub enum IsbnError {
    WrongChecksum,
    InvalidCharacter(usize, char),
}

impl FromStr for Isbn {
    type Err = IsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<u8>  = Vec::with_capacity(13);

        for (i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => return Err(IsbnError::InvalidCharacter(i, c)),
            }
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(IsbnError::WrongChecksum)
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits: digits,
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];

    let sum: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&x, &y)| x * y)
        .map(|it| it as u32)
        .sum();

    let digit = 10 - (sum % 10);

    match digit {
        10 => 0_u8,
        x => x as u8,
    }
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
