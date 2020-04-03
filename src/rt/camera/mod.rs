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

/// An orthograpich camera.
pub struct OrthographicCamera {
    cam: GenericCamera,
}

pub struct PerspectiveCamera {
    cam: GenericCamera,
}

impl OrthographicCamera {
    /// Creates a new orthographic camera.
    pub fn new(o: Point, d: Vector, up: Vector, scale_x: f64, scale_y: f64) -> Self {
        let x_span = d.cross(up) * scale_x * 0.5;
        let y_span = x_span.cross(d) * scale_y * 0.5;
        Self {
            cam: GenericCamera { o, d, up, x_span, y_span }
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_primary_ray(self, x: f64, y: f64) -> Ray {
        let v = x * self.cam.x_span
              + y * self.cam.y_span;
        Ray::new(self.cam.o + v, self.cam.d)
    }
}

impl PerspectiveCamera {
    /// Creates a new perspective camera.
    pub fn new(o: Point, d: Vector, up: Vector, h: f64, v: f64) -> Self {
        let x_span = d.cross(up).norm() * f64::tan(h / 2.0);
        let y_span = x_span.cross(d).norm() * f64::tan(v / 2.0);
        Self {
            cam: GenericCamera { o, d, up, x_span, y_span }
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_primary_ray(self, x: f64, y: f64) -> Ray {
        let v = x * self.cam.x_span + y * self.cam.y_span;
        Ray::new(self.cam.o, (self.cam.d + v).norm())
    }
}
