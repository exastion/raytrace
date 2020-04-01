use crate::core::Point;
use crate::core::Vector;
use crate::rt::Ray;

trait Camera {
    fn get_primary_ray(self, x: f64, y: f64) -> Ray;
}

pub struct GenericCamera {
    o: Point,
    d: Vector,
    up: Vector,
    x_span: Vector,
    y_span: Vector,
}

pub struct OrthographicCamera {
    base: GenericCamera,
}

impl OrthographicCamera {
    pub fn new(o: Point, d: Vector, up: Vector, scale_x: f64, scale_y: f64) -> Self {
        let x_span = d.cross(up) * scale_x * 0.5;
        let y_span = x_span.cross(d) * scale_y * 0.5;
        Self {
            base: GenericCamera { o, d, up, x_span, y_span }
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_primary_ray(self, x: f64, y: f64) -> Ray {
        let v = x * self.base.x_span
              + y * self.base.y_span;
        Ray::new(self.base.o + v, self.base.d)
    }
}
