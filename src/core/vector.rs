use std::iter;
use std::ops;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn rep(v: f64) -> Self {
        Self::new(v, v, v)
    }

    fn components(&self) -> impl Iterator<Item = f64> {
        let x = iter::once(self.x);
        let y = iter::once(self.y);
        let z = iter::once(self.z);
        x.chain(y).chain(z)
    }

    pub fn len(self) -> f64 {
       self.lensq().sqrt() 
    }

    pub fn lensq(self) -> f64 {
        self.components()
            .map(|v| v.powi(2))
            .sum()
    }

    pub fn norm(self) -> Self {
        self / self.len()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vector { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn components() {
        let v = Vector::new(1.0, -3.0, 1000.0);
        let mut c = v.components();
        assert_eq!(c.next(), Some(1.0));
        assert_eq!(c.next(), Some(-3.0));
        assert_eq!(c.next(), Some(1000.0));
        assert_eq!(c.next(), None);
    }

    #[test]
    fn len() {
        let v1 = Vector::new(4.0, 3.0, 0.0);
        let v2 = Vector::new(234.0, 57.0, -4576.0);
        assert_eq!(v1.len(), 5.0);
        assert_eq!(v2.len(), ((234*234 + 57*57 + 4576*4576) as f64).sqrt());
    }

    #[test]
    fn lensq() {
        let v = Vector::new(346.0, 135.0, 535.0);
        assert_eq!(v.lensq(), (346*346 + 135*135 + 535*535) as f64);
    }
    #[test]
    fn norm() {
        let v = Vector::new(23456.0, 6543.0, 2345.0);
        let n = v.norm();
        assert_eq!(n.len(), 1.0);
        assert_eq!(n * v.len(), v);
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(1.0, 3.0, 4.0);
        let v2 = Vector::new(3.0, 10.0, 50.0);
        let v3 = Vector::new(-22.0, -4.0, 3.0);
        assert_eq!(v1.cross(v1), Vector::rep(0.0));
        assert_eq!(v2.cross(v2), Vector::rep(0.0));
        assert_eq!(v3.cross(v3), Vector::rep(0.0));
        assert_eq!(v1.cross(v2), Vector::new(110.0, -38.0, 1.0));
        assert_eq!(v2.cross(v3), Vector::new(230.0, -1109.0, 208.0));
        assert_eq!(v3.cross(v1), Vector::new(-25.0, 91.0, -62.0));
    }

    #[test]
    fn neg() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(-v, Vector { x: -1.0, y: -2.0, z: -3.0 });
    }

    #[test]
    fn add() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 4.0, 9.0);
        let mut v3 = Vector::new(2.0, 6.0, 8.0);
        v3 += v2 + v1;
        assert_eq!(v1 + v2, Vector { x: 3.0, y: 6.0, z: 12.0 });
        assert_eq!(v3, Vector { x: 5.0, y: 12.0, z: 20.0 });
    }

    #[test]
    fn sub() {
        let v1 = Vector::new(1.0, 5.0, 9.0);
        let v2 = Vector::new(2.0, 0.0, 3.0);
        let mut v3 = Vector::new(4.0, 1.0, 5.0);
        v3 -= v2 - v1;
        assert_eq!(v1 - v2, Vector { x: -1.0, y: 5.0, z: 6.0 });
        assert_eq!(v3, Vector { x: 3.0, y: 6.0, z: 11.0 });
    }

    #[test]
    fn mul() {
        let v1 = Vector::new(1.0, 5.0, 6.0);
        let mut v2 = Vector::new(3.0, 2.0, 9.0);
        v2 *= 1.25;
        assert_eq!(v1 * 500.0, Vector { x: 500.0, y: 2500.0, z: 3000.0 });
        assert_eq!(500.0 * v1, Vector { x: 500.0, y: 2500.0, z: 3000.0 });
        assert_eq!(v2, Vector { x: 3.75, y: 2.5, z: 11.25 });
    }

    #[test]
    fn div() {
        let v1 = Vector::new(1.0, 4.0, 2.0);
        let mut v2 = Vector::new(3.0, 7.0, 2.0);
        v2 /= 10.0;
        assert_eq!(v1 / 0.5, Vector { x: 2.0, y: 8.0, z: 4.0 });
        assert_eq!(v2, Vector { x: 0.3, y: 0.7, z: 0.2 });
    }
}
