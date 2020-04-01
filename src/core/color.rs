use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn rep(v: f64) -> Self {
        Self::new(v, v, v)
    }

    pub fn clamp(self) -> Self {
        Self {
            r: 1f64.min(0f64.max(self.r)),
            g: 1f64.min(0f64.max(self.g)),
            b: 1f64.min(0f64.max(self.b))
        }
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b
        }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }
    }
}

impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs
        }
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b
        }
    }
}

impl ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs
        }
    }
}

impl ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = Color::new(1.0, 0.5, 0.3333);
        assert_eq!(c, Color { r: 1.0, g: 0.5, b: 0.3333 });
    }

    #[test]
    fn clamp() {
        let c = Color::new(-1.0, 10.0, 0.5);
        assert_eq!(c.clamp(), Color { r: 0.0, g: 1.0, b: 0.5 })
    }

    #[test]
    fn add() {
        let c1 = Color::new(0.5, 0.45, 0.221);
        let c2 = Color::new(0.3, 0.765, 0.123);
        let mut c3 = Color::new(0.111, 0.432, 0.113);
        c3 += c1;
        assert_eq!(c1 + c2, Color { r: 0.8, g: 1.215, b: 0.344 });
        assert_eq!(c3, Color { r: 0.611, g: 0.882, b: 0.334 });
    }

    #[test]
    fn sub() {
        let c1 = Color::new(1.0, 0.34, 0.67);
        let c2 = Color::new(0.2, 0.14, 0.67);
        let mut c3 = Color::new(0.3, 0.54, 0.67);
        c3 -= c1;
        assert_eq!(c1 - c2, Color { r: 0.8, g: 0.2, b: 0.0 });
        assert_eq!(c3, Color { r: -0.7, g: 0.2, b: 0.0 });
    }

    #[test]
    fn mul() {
        let c1 = Color::new(1.0, 5.0, 6.0);
        let mut c2 = Color::new(3.0, 2.0, 9.0);
        c2 *= 1.25;
        assert_eq!(c1 * 500.0, Color { r: 500.0, g: 2500.0, b: 3000.0 });
        assert_eq!(500.0 * c1, Color { r: 500.0, g: 2500.0, b: 3000.0 });
        assert_eq!(c2, Color { r: 3.75, g: 2.5, b: 11.25 });
    }

    #[test]
    fn div() {
        let c1 = Color::new(1.0, 4.0, 2.0);
        let mut c2 = Color::new(3.0, 7.0, 2.0);
        c2 /= 10.0;
        assert_eq!(c1 / 0.5, Color { r: 2.0, g: 8.0, b: 4.0 });
        assert_eq!(c2, Color { r: 0.3, g: 0.7, b: 0.2 });
    }
}
