use crate::core::Vector;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn rep(v: f64) -> Self {
        Self::new(v, v, v)
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: f64::min(self.x, other.x),
            y: f64::min(self.y, other.y),
            z: f64::min(self.z, other.z),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: f64::max(self.x, other.x),
            y: f64::max(self.y, other.y),
            z: f64::max(self.z, other.z),
        }
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::MulAssign<f64> for Point {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p, Point { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn min_max() {
        let p1 = Point::new(1.5, 2.8, 3.5);
        let p2 = Point::new(-3.0, 10.5, 5.5);
        assert_eq!(Point::min(p1, p2), Point { x: -3.0, y: 2.8, z: 3.5 });
        assert_eq!(Point::max(p1, p2), Point { x: 1.5, y: 10.5, z: 5.5 });
    }

    #[test]
    fn add() {
        let p = Point::new(1.0, 6.0, 15.0);
        let v = Vector::new(4.0, -2.0, 4.0);
        assert_eq!(p + v, Point { x: 5.0, y: 4.0, z: 19.0 });
    }

    #[test]
    fn sub() {
        let p1 = Point::new(1.0, 5.0, 9.0);
        let p2 = Point::new(2.0, 0.0, 3.0);
        assert_eq!(p1 - p2, Vector { x: -1.0, y: 5.0, z: 6.0 });
    }

    #[test]
    fn mul() {
        let p1 = Point::new(1.0, 5.0, 6.0);
        let mut p2 = Point::new(3.0, 2.0, 9.0);
        p2 *= 1.25;
        assert_eq!(p1 * 500.0, Point { x: 500.0, y: 2500.0, z: 3000.0 });
        assert_eq!(500.0 * p1, Point { x: 500.0, y: 2500.0, z: 3000.0 });
        assert_eq!(p2, Point { x: 3.75, y: 2.5, z: 11.25 });
    }

    #[test]
    fn div() {
        let p1 = Point::new(1.0, 4.0, 2.0);
        let mut p2 = Point::new(3.0, 7.0, 2.0);
        p2 /= 10.0;
        assert_eq!(p1 / 0.5, Point { x: 2.0, y: 8.0, z: 4.0 });
        assert_eq!(p2, Point { x: 0.3, y: 0.7, z: 0.2 });
    }
}
