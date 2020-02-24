use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Vec3f {
    pub fn norm(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(&self) -> Vec3f {
        self.clone() * (1f32 /self.norm())
    }
}

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

impl Mul<Vec3f> for Vec3f {
    type Output = f32;
    fn mul(self, rhs: Vec3f) -> f32 {
        let mut ret = 0f32;
        ret += self.0 * rhs.0;
        ret += self.1 * rhs.1;
        ret += self.2 * rhs.2;
        ret
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Vec3f {
        Vec3f(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

#[derive(Clone)]
pub struct Material {
    pub diffuse_color : Vec3f
}

impl Material {
    pub fn new(diffuse_color: Vec3f) -> Material {
        Material { diffuse_color }
    }
}

pub struct Sphere {
    center: Vec3f,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32, material: Material) -> Sphere {
        Sphere { center, radius, material}
    }

    pub fn get_material(&self) -> Material {
        self.material.clone()
    }
}

pub trait Ray {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<f32>;
}

impl Ray for Sphere {
    fn intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<f32> {
        let L = self.center.clone() - orig;
        let tca = L.clone() * dir.clone();
        let d2 = L.clone() * L.clone() - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }

        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0f32 {
            t0 = t1;
        }
        if t0 < 0f32 {
            return None;
        }
        Some(t0)
    }
}
