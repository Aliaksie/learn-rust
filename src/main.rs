use std::char;

fn main() {
    // works_with_primitive();

    // works_with_array();

    // works_with_tuple();

    // let result = square(8);
    // println!("Results is {:?}", result);

    // assert_eq!(celsius_to_fahrenheit(23.0), 73.4);
    // print!("It works!");

    // works_with_loop();

    // feild shadowing
    // work_with_shadowing();

    // ownership
    // work_with_ownership();

    // work_int_ownership();
    let rocket_flue = String::from("RP-1");
    // process_flue_str(rocket_flue);
    // println!("After process {}", rocket_flue); // not owner any more!!
    let rocket_flue = process_flue_str(rocket_flue);
    println!("After process {}", rocket_flue);
}

// stack!!
fn work_int_ownership() {
    let rocket_flue = 1;
    process_flue_int(rocket_flue);
    println!("After process {}", rocket_flue);
}

fn process_flue_str(mut propelant: String) -> String {
    propelant.push_str("...");
    println!("Processing {}", propelant);
    String::from("LNG")
}

fn process_flue_int(mut propelant: i32) {
    propelant += 1;
    println!("Processing {}", propelant);
}

fn work_with_ownership() {
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

fn work_with_shadowing() {
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

fn works_with_loop() {
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

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (1.8 * temp) + 32.0
}

fn square(x: i32) -> (i32, i32) {
    println!("Input is {}", x);
    (x * x, x)
}

fn works_with_tuple() {
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
