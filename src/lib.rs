pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

pub enum Feature {
    Area,
    Perimeter,
}

impl Shape {
    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter(),
        }
    }

    fn calc_area(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r.powi(2) * std::f64::consts::PI,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                let p: f64 = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).abs().sqrt()
            }
        }
    }

    fn calc_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_rectangle() {
        let w = 1.2; // width
        let h = 3.4; // height
        let rect = Shape::Rectangle(w, h);

        // test public functions
        let area = rect.get_feature(Feature::Area);
        assert_eq!(area, w * h);

        let perimeter = rect.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 2.0 * w + 2.0 * h);

        // test private functions
        let area = rect.calc_area();
        assert_eq!(area, w * h);

        let perimeter = rect.calc_perimeter();
        assert_eq!(perimeter, 2.0 * w + 2.0 * h);
    }

    #[test]
    fn ut_circle() {
        let r = 5.6; // radius
        let circ = Shape::Circle(r);

        // test public functions
        let area = circ.get_feature(Feature::Area);
        assert_eq!(area, std::f64::consts::PI * r.powi(2));

        let perimeter = circ.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 2.0 * std::f64::consts::PI * r);

        // test private functions
        let area = circ.calc_area();
        assert_eq!(area, std::f64::consts::PI * r.powi(2));

        let perimeter = circ.calc_perimeter();
        assert_eq!(perimeter, 2.0 * std::f64::consts::PI * r);
    }

    #[test]
    fn ut_triangle() {
        let a = 3.0;
        let b = 4.0;
        let c = 5.0;
        let triangle = Shape::Triangle(a, b, c);

        let p: f64 = (a + b + c) / 2.0;
        let ex_area = (p * (p - a) * (p - b) * (p - c)).sqrt();

        // test public functions
        let area = triangle.get_feature(Feature::Area);
        assert_eq!(area, ex_area);

        let perimeter = triangle.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, a + b + c);

        // test private functions
        let area = triangle.calc_area();
        assert_eq!(area, ex_area);

        let perimeter = triangle.calc_perimeter();
        assert_eq!(perimeter, a + b + c);
    }
}
