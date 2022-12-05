use learn_rust::{Feature, Shape};

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    let area = rect.get_feature(Feature::Area);
    println!("rect area is {}", area);

    let circ = Shape::Circle(3.0);
    let perimeter = circ.get_feature(Feature::Perimeter);
    println!("circ perimeter is {}", perimeter);

}
