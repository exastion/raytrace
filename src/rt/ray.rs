use crate::core::Point;
use crate::core::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    o: Point,
    d: Vector,
}

impl Ray {
    pub fn new(o: Point, d: Vector) -> Self {
        Self { o, d: d }
    }

    pub fn get_point(self, distance: f64) -> Point {
        self.o + self.d * distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let o = Point::new(0.0, 1.0, 5.0);
        let d = Vector::new(1.1, 2.2, 3.3);
        let r = Ray::new(o, d);
        assert_eq!(r, Ray { o, d });
    }

    #[test]
    fn get_point() {
        let o = Point::rep(0.0);
        let d = Vector::rep(1.0).norm();
        let r = Ray::new(o, d);
        let p = r.get_point(10.0);
        assert!((p - o).len() - 10.0 < 10.0*std::f64::EPSILON);
    }
}
