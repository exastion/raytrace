pub use self::renderer::Renderer;
pub use self::ray::Ray;
mod renderer;
mod ray;
mod camera;

use crate::core::Point;
use crate::core::Vector;

pub struct BBox {
    min: Point,
    max: Point,
}

impl BBox {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self::new(Point::rep(std::f64::MAX), Point::rep(-std::f64::MAX))
    }

    pub fn full() -> Self {
        Self::new(Point::rep(-std::f64::MAX), Point::rep(std::f64::MAX))
    }

    pub fn extend(&mut self, other: BBox) {
        self.min = Point::min(self.min, other.min);
        self.max = Point::max(self.max, other.max);
    }

    pub fn intersect(self, ray: Ray) -> (f64, f64) {
        let t0 = (self.min.x - ray.o.x) / ray.d.x;
        let t1 = (self.max.x - ray.o.x) / ray.d.x;
        let tx0 = f64::min(t0, t1);
        let tx1 = f64::max(t0, t1);
        let t0 = (self.min.y - ray.o.y) / ray.d.y;
        let t1 = (self.max.y - ray.o.y) / ray.d.y;
        let ty0 = f64::min(t0, t1);
        let ty1 = f64::max(t0, t1);
        let t0 = (self.min.z - ray.o.z) / ray.d.z;
        let t1 = (self.max.z - ray.o.z) / ray.d.z;
        let tz0 = f64::min(t0, t1);
        let tz1 = f64::max(t0, t1);
        (tx0.max(ty0).max(tz0), tx1.min(ty1).min(tz1))
    }

    pub fn is_unbound(self) -> bool {
        self.max.x.max(self.max.y).max(self.max.z) == std::f64::MAX
            || self.min.x.min(self.min.y).min(self.min.z) == -std::f64::MIN
    }

    pub fn contains(self, other: BBox) -> bool {
        Point::min(self.min, other.min) == self.min
            && Point::max(self.max, other.max) == self.max
    }

    pub fn contains_point(self, p: Point) -> bool {
        Point::min(self.min, p) == self.min
            && Point::max(self.max, p) == self.max
    }

    pub fn is_empty(self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }
}

pub struct Intersection {
    ray: Ray,
    //solid: Solid,
    distance: f64,
    n: Vector,
    uv: Point,
}

impl Intersection {
    pub fn new(distance: f64, ray: Ray, /*solid: Solid, */n: Vector, uv: Point) -> Self {
        Self { ray, distance, n, uv } 
    }

    pub fn hit_point(self) -> Point {
        self.ray.get_point(self.distance)
    }

    pub fn normal(self) -> Vector { self.n }

    pub fn local(self) -> Point { self.uv }
}

trait Primitive {
    fn get_bounds() -> BBox;
    fn intersect(ray: Ray, previousBestDistance: f64) -> Option<Intersection>;
    // fn set_material(m: Material);
    // fn set_coord_mapper(cm: CoordMapper);
}
