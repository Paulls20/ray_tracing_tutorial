use std::ops::*;

#[derive(Debug, Clone)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

pub struct Sphere {
    center: Vec3f,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32) -> Sphere {
        Sphere { center, radius}
    }
}

pub trait Ray {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<f32>;
}

impl Ray for Sphere {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<f32> {
        println!("Test intersect");
        None
    }
}
