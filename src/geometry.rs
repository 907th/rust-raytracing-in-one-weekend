use std::ops::{Add, Neg, Sub};

pub(crate) const EPS :f64 = 1e-13;

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Vector(pub(crate) f64, pub(crate) f64, pub(crate) f64);

pub(crate) type Point = Vector;
pub(crate) type Color = Vector;

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Vector,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Sphere {
    pub(crate) center: Point,
    pub(crate) radius: f64,
}

pub(crate) trait Crossing {
    fn cross_point(self: Self, ray: Ray) -> Option<Point>;
}

pub(crate) trait Normal {
    fn normal_at(self: Self, point: Point) -> Vector;
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Vector {
    pub(crate) fn origin() -> Self {
        Self::default()
    }

    pub(crate) fn new(a: f64, b: f64, c: f64) -> Self {
        Self(a, b, c)
    }

    pub(crate) fn scale(self, factor: f64) -> Self {
        Self(self.0 * factor, self.1 * factor, self.2 * factor)
    }

    pub(crate) fn shrink(self, factor: f64) -> Self {
        Self(self.0 / factor, self.1 / factor, self.2 / factor)
    }

    pub(crate) fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub(crate) fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn unit(self) -> Self {
        let l = self.length();
        Self(self.0 / l, self.1 / l, self.2 / l)
    }

    pub(crate) fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub(crate) fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl Ray {
    pub(crate) fn new(origin: Point, direction: Vector) -> Self {
        Self{ origin, direction }
    }

    pub(crate) fn at(self, t: f64) -> Point {
        self.origin + self.direction.scale(t)
    }
}

impl Crossing for Sphere {
    fn cross_point(self: Self, ray: Ray) -> Option<Point> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < -EPS { return None; }
        let d = if discriminant < EPS { 0.0 } else { discriminant };
        let t1 = (-b + d.sqrt()) / (2.0 * a);
        let t2 = (-b - d.sqrt()) / (2.0 * a);
        let p1 = ray.at(t1);
        let p2 = ray.at(t2);
        if (p1 - ray.origin).length() < (p2 - ray.origin).length() { Some(p1) } else { Some(p2) }
    }
}

impl Normal for Sphere {
    fn normal_at(self: Self, point: Point) -> Vector {
        (point - self.center).unit()
    }
}
