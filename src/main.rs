mod challenges;

fn main() {
    let rust_in_action: challenges::isbn::Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}
