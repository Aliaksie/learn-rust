use std::ops::{Add, Deref};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn magnitude(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }

    fn dist(&self, point: &Point) -> f64 {
        f64::from((self.x - point.x).pow(2) + (self.y - point.y).pow(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Deref for Polygon {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points: vec![] }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    fn left_most_point(&self) -> Option<Point> {
        let binding = Point::new(0, 0);
        let mut left_point = self.points.first().unwrap_or(&binding);
        for point in self.points.iter() {
            if left_point.x > point.x || (left_point.x == point.x && left_point.y > point.y) {
                left_point = point
            }
        }

        Some(*left_point)
    }
}

pub struct Circle {
    point: Point,
    r: i32,
}

impl Circle {
    fn new(point: Point, r: i32) -> Circle {
        Circle { point: point, r: r }
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn circumference(&self) -> f64 {
        match self {
            Shape::Polygon(polygon) => polygon
                .iter()
                .zip(polygon.iter().cycle().skip(1))
                .map(|f| f.0.dist(f.1))
                .reduce(|a, b| (a + b))
                .unwrap(), // we dont worry
            Shape::Circle(сircle) => ((2 * сircle.r) as f64) * std::f64::consts::PI,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));

        let circle = Circle::new(Point::new(10, 20), 5);

        let shapes = vec![
            Shape::from(Shape::Polygon(poly)), //
            Shape::from(Shape::Circle(circle)),
        ];

        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        println!("{:?}", circumferences);
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}