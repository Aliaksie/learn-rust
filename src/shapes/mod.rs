
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
                (p * (p - a) * (p - b) * (p - c)).sqrt()
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
