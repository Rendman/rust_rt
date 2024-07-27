use std::ops::{self, Range};

use rand::prelude::*;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Vec3 {

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(self) -> Vec3{
        self / self.length()
    }

    pub fn dot(self, other: Vec3) -> f64{
         self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn random() -> Vec3{
        let mut rng = thread_rng();
        let xr : f64 = rng.gen();
        let yr : f64 = rng.gen();
        let zr : f64 = rng.gen();

        Vec3{x: xr, y: yr, z: zr}
    }

    pub fn random_bounds(range: Range<f64>) -> Vec3 {
        let mut rng = thread_rng();
        let xr : f64 = rng.gen_range(range.clone());
        let yr : f64 = rng.gen_range(range.clone());
        let zr : f64 = rng.gen_range(range.clone());

        Vec3{x: xr, y: yr, z: zr}
    }

    pub fn random_in_unit_sphere(self) -> Vec3 {
        loop {
            let p = Vec3::random_bounds(-1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(self) -> Vec3 {
        self.random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(self) -> Vec3 {
        let on_unit_sphere = self.random_unit_vector();
        if on_unit_sphere.dot(self) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = thread_rng(); 
        loop {
            let p = Vec3{x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: 0.0};
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(self) -> bool {
        return f64::abs(self.x) < f64::EPSILON && f64::abs(self.y) < f64::EPSILON && f64::abs(self.z) < f64::EPSILON;
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3{ x: self.y * other.z - self.z * other.y,
              y: self.z * other.x - self.x * other.z,
              z: self.x * other.y - self.y * other.x   
        }
    }

}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs;
    }
}
