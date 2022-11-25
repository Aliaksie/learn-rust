fn main() {
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
    println!("value in bit is {:08b}, after apply AND", !value & 0b1111_0111u8);
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
