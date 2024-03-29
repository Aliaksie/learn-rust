use std::fmt;
use std::fs;
use std::io;
use std::mem;

use rand::prelude::*;

pub struct Color(u8, u8, u8); // RGB
pub struct Point(u8, u8, u8); // XYZ

#[derive(Debug, Clone)]
pub struct Shuttle<T> {
    name: String,
    crew: T,
    propellant: f64,
}

impl<T> Shuttle<T> {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_flue(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    pub fn new(name: &str, size: T) -> Shuttle<T> {
        Shuttle {
            name: String::from(name),
            crew: size,
            propellant: 100.0,
        }
    }
}

pub fn lifetime_fluet<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn print_compare<T, U>(left: T, right: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if left == T::from(right) {
        println!("{} eq to {}", left, right);
    } else {
        println!("{} not_eq to {}", left, right);
    }
}

pub fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}

pub fn guess_the_number() {
    let number: u32 = thread_rng().gen_range(1..100);
    println!("I gueesed the secret number from 1..100. Try to guess it!");
    let mut numbers: Vec<u32> = Vec::new();
    let mut tries: usize = 4;
    loop {
        let mut buffer = String::new();
        let input_number = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Wrong input. Pleas try again!");
                    continue;
                }
            },
            Err(_) => {
                println!("Wrong input. Pleas try again!");
                continue;
            }
        };

        numbers.push(input_number);
        let tip = if number == input_number {
            break;
        } else if number > input_number {
            "lower"
        } else {
            "high"
        };
        if tries == 0 {
            panic!(
                "Opps. You lose the game :(. I guessed the number {}!",
                number
            );
            break;
        }
        println!(
            "Your numer is {}, too {}, Please try again!. Tries left {}",
            input_number, tip, tries
        );
        buffer.clear();
        tries -= 1;
    }
    println!(
        "You got it! I guessed the number {}! You inserted {:?}",
        number, numbers
    );
}

pub fn works_with_slices() {
    let msg = String::from("Greetings from Poland!");
    println!("Msg   is {}", msg);
    let last_world = &msg[15..15 + 6];
    println!("Last  is {}", last_world);
    let fist = get_first_world(&msg);
    println!("First is {}", fist);
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

pub fn trim_spaces(s: &str) -> &str {
    let bytes = s.as_bytes();
    if bytes.len() == 0 {
        return &s;
    }

    let mut start: usize = 0;
    let mut end: usize = bytes.len() - 1;

    while start != end {
        if bytes[start] == b' ' {
            start += 1;
        }

        if bytes[end] == b' ' {
            end -= 1;
        }

        if bytes[start] != b' ' && bytes[end] != b' ' {
            break;
        }
    }

    if start != end {
        end += 1;
    }

    &s[start..end]
}

pub fn get_first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}

// stack!!
pub fn work_int_ownership() {
    let rocket_flue = 1;
    process_flue_int(rocket_flue);
    println!("After process {}", rocket_flue);
}

pub fn process_flue_str(propelant: &mut String) -> usize {
    propelant.push_str("...");
    println!("Processing {}", propelant);

    propelant.len()
}

pub fn process_flue_int(mut propelant: i32) {
    propelant += 1;
    println!("Processing {}", propelant);
}

pub fn work_with_ownership() {
    let out_planet;
    let clone_planet;
    {
        let inner_planet = String::from("Mercury");
        println!("Inner {}", inner_planet);
        out_planet = inner_planet;
        // inner_planet ownersiph was move! removed from stack
    }
    clone_planet = out_planet.clone();
    // copy heap value and owned it
    println!("Out {}", out_planet);
    println!("Out {}", clone_planet);
}

pub fn work_with_shadowing() {
    let planet = "Earth";
    {
        println!("Planet is {}", planet);
        let planet = 4; // Mars :)
        println!("Planet is {}", planet);
    }
    println!("Planet is {}", planet);
    print!("It works!");
    // string type, dynamic alocate memory
    let mut msg = String::from("Earth");
    msg.push_str(" is home");
    println!("{}", msg);
}

pub fn works_with_loop() {
    let mut count = 0;
    let result = loop {
        if count == 5 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };
    println!("Result after loop is {}", result);
    let letters = ['a', 'b', 'c', 'd'];
    for item in letters {
        println!("Item is {}", item);
    }
    for (index, &item) in letters.iter().enumerate() {
        println!("Item {} is {}", index, item);
        if item == 'b' {
            break;
        }
    }
    for number in 0..5 {
        println!("Number is {}", number);
    }
    let mut matrix = [[1, 2, 3], [3, 4, 5], [4, 5, 6]];
    for row in matrix.iter_mut() {
        for element in row.iter_mut() {
            *element += 1;
            print!("{}\t", element);
        }
        println!();
    }
    let array = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = -2147483648;
    let mut min: i32 = 2147483647;
    let mut mean: f64 = 0.0;
    for value in array {
        mean += value as f64;
        if value > max {
            max = value;
        }
        if min > value {
            min = value;
        }
    }
    mean /= array.len() as f64;
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
}

pub fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (1.8 * temp) + 32.0
}

pub fn square(x: i32) -> (i32, i32) {
    println!("Input is {}", x);
    (x * x, x)
}

pub fn works_with_tuple() {
    let stuff: (u8, f32, char) = (10, 3.14, 'x');
    println!("Tuple  is {:?}", stuff);
    println!("Second is {}", stuff.1);
    let (a, b, c) = stuff;
    println!("A is {}", a);
}

pub fn works_with_array() {
    let letters = ['a', 'b', 'c', 'd'];
    let first = letters[0];
    println!("Array is {:?}", letters);
    println!("First element {}", first);
    let mut numbers: [i32; 10];
    // print!("error {}", numbers[0]);
    numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Array is {:?}", numbers);
    println!("First element {}", numbers[0]);
    numbers = [1; 10];
    println!("Array is {:?}", numbers);
    println!("First element {}", numbers[0]);
    let parking_lots = [[1, 2, 3], [4, 5, 6]];
    println!("Array[][] is {:?}", parking_lots);
    println!("First lot is {:?}", parking_lots[0][0]);
    let garage: [[[i32; 100]; 10]; 5];
    garage = [[[1; 100]; 10]; 5];
    println!("Array[][][] is {:?}", garage);
}

pub fn works_with_primitive() {
    let a = 4;
    let b = 10.0;
    println!("sum = {}", a as f64 + b);
    println!("minus = {}", a as f64 - b);
    println!("multys = {}", a as f64 * b);
    println!("divide = {}", b / a as f64);
    println!("divide = {}", a as f64 / b);
    println!("mod = {}", b % a as f64);
    println!("Bitwise operation");
    let value = 0b1111_0101u8;
    println!("value in int is {}", value);
    println!("value in bit is {:08b}", value);
    println!("value in bit is {:08b}, after apply NOT", !value);
    println!(
        "value in bit is {:08b}, after apply AND",
        !value & 0b1111_0111u8
    );
    println!("bit 3 is {}", value & 0b0000_1000u8);
    println!("bit 2 is {}", value & 0b0000_0100u8);
    println!("value is {:08b}, OR", !value | 0b0100_0000u8);
    println!("value is {:08b}, XOR", !value ^ 0b0101_0101u8);
    println!("value is {:08b}, <<", !value << 4);
    println!("value is {:08b}, >>", !value >> 2);
    let _true = true;
    let _false = false;
    println!("NOT is {} ", !_true);
    println!("AND is {} ", _true & _false);
    println!("OR  is {} ", _true | _false);
    println!("XOR is {} ", _true ^ _false);
    println!("    is {} ", (_true ^ _false) | (_true & _false));
    println!("    is {} ", (_true ^ _false) || panic!());
    let letter = 'a';
    let number = '1';
    let finger = '\u{261d}';
    println!("{0}, {1}, {2}", letter, number, finger);
}