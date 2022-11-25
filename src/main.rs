use std::char;

fn main() {
    // works_with_primitive();

    // works_with_array();

    let stuff: (u8, f32, char) = (10, 3.14, 'x');
    println!("Tuple  is {:?}", stuff);
    println!("Second is {}", stuff.1);

    let (a, b, c) = stuff;
    println!("A is {}", a);
}

fn works_with_array() {
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

fn works_with_primitive() {
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
