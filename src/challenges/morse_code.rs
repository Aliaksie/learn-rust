// https://en.wikipedia.org/wiki/Morse_code

#[derive(Debug, PartialEq)]
pub enum Signal {
    Dot,
    Dashe,
}

type Letter = Vec<Signal>;
pub type Message = Vec<Letter>;

pub trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Signal::*;

        let mut msg = Vec::with_capacity(self.len());
        for c in self.chars() {
            let letter = match c {
                'A' | 'a' => vec![Dot, Dashe],
                'B' | 'b' => vec![Dashe, Dot, Dot, Dot],
                'C' | 'c' => vec![Dashe, Dot, Dashe, Dot],
                'D' | 'd' => vec![Dashe, Dot, Dot],
                'E' | 'e' => vec![Dot],
                'F' | 'f' => vec![Dot, Dot, Dashe, Dot],
                'G' | 'g' => vec![Dashe, Dashe, Dot],
                'H' | 'h' => vec![Dot, Dot, Dot, Dot],
                'I' | 'i' => vec![Dot, Dot],
                'J' | 'j' => vec![Dot, Dashe, Dashe, Dashe],
                'K' | 'k' => vec![Dashe, Dot, Dashe],
                'L' | 'l' => vec![Dot, Dashe, Dot, Dot],
                'M' | 'm' => vec![Dashe, Dashe],
                'N' | 'n' => vec![Dashe, Dot],
                'O' | 'o' => vec![Dashe, Dashe, Dashe],
                'P' | 'p' => vec![Dot, Dashe, Dashe, Dot],
                'Q' | 'q' => vec![Dashe, Dashe, Dot, Dashe],
                'R' | 'r' => vec![Dot, Dashe, Dot],
                'S' | 's' => vec![Dot, Dot, Dot],
                'T' | 't' => vec![Dashe],
                'U' | 'u' => vec![Dot, Dot, Dashe],
                'V' | 'v' => vec![Dot, Dot, Dot, Dashe],
                'W' | 'w' => vec![Dot, Dashe, Dashe],
                'X' | 'x' => vec![Dashe, Dot, Dot, Dashe],
                'Y' | 'y' => vec![Dashe, Dot, Dashe, Dashe],
                'Z' | 'z' => vec![Dashe, Dashe, Dot, Dot],

                '1' => vec![Dot, Dashe, Dashe, Dashe, Dashe],
                '2' => vec![Dot, Dot, Dashe, Dashe, Dashe],
                '3' => vec![Dot, Dot, Dot, Dashe, Dashe],
                '4' => vec![Dot, Dot, Dot, Dot, Dashe],
                '5' => vec![Dot, Dot, Dot, Dot, Dot],
                '6' => vec![Dashe, Dot, Dot, Dot, Dot],
                '7' => vec![Dashe, Dashe, Dot, Dot, Dot],
                '8' => vec![Dashe, Dashe, Dashe, Dot, Dot],
                '9' => vec![Dashe, Dashe, Dashe, Dashe, Dot],
                '0' => vec![Dashe, Dashe, Dashe, Dashe, Dashe],
                _ => continue,
            };

            msg.push(letter);
        }
        msg
    }
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::Dot => write!(f, "."),
            Signal::Dashe => write!(f, "_"),
        }
    }
}

pub fn print(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

#[test]
fn hello_world() {
    use Signal::*;

    let expected = vec![
        vec![Dot, Dot, Dot, Dot],
        vec![Dot],
        vec![Dot, Dashe, Dot, Dot],
        vec![Dot, Dashe, Dot, Dot],
        vec![Dashe, Dashe, Dashe],
        vec![Dot, Dashe, Dashe],
        vec![Dashe, Dashe, Dashe],
        vec![Dot, Dashe, Dot],
        vec![Dot, Dashe, Dot, Dot],
        vec![Dashe, Dot, Dot],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
