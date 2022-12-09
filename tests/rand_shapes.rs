use rand::prelude::*;
use learn_rust::*;

#[test]
fn it_rand_rectangle() {
    let mut rng = thread_rng();
    let w = rng.gen_range(0.0..10.0); // width
    let h = rng.gen_range(0.0..10.0); // height

    let rect = Shape::Rectangle(w, h);

    let area = rect.get_feature(Feature::Area);
    assert_eq!(area, w * h);

    let perimeter = rect.get_feature(Feature::Perimeter);
    assert_eq!(perimeter, 2.0 * w + 2.0 * h);
}

#[test]
fn it_rand_circle() {
    let mut rng = thread_rng();
    let r = rng.gen_range(0.0..10.0); // radius

    let circ = Shape::Circle(r);

    let area = circ.get_feature(Feature::Area);
    assert_eq!(area, std::f64::consts::PI * r.powi(2));

    let perimeter = circ.get_feature(Feature::Perimeter);
    assert_eq!(perimeter, 2.0 * std::f64::consts::PI * r);
}

#[test]
fn it_rand_triangle() {
    let mut rng = thread_rng();
    let a = rng.gen_range(0.0..3.0);
    let b = rng.gen_range(a..4.0);
    let c = rng.gen_range(b..5.0);
    let triangle = Shape::Triangle(a, b, c);

    let p: f64 = (a + b + c) / 2.0;
    let ex_area = (p * (p - a) * (p - b) * (p - c)).abs().sqrt();

    // test public functions
    let area = triangle.get_feature(Feature::Area);
    assert_eq!(area, ex_area);

    let perimeter = triangle.get_feature(Feature::Perimeter);
    assert_eq!(perimeter, a + b + c);
}
