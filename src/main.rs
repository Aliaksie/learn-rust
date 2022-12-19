use crate::challenges::morse_code::*;

mod challenges;

fn main() {
    let greeting = "Hello, world"
        .to_string()
        .to_morse_code();

    print(&greeting);
}
