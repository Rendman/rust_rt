use rand::random;

use crate::{hit::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (n * 2.0 * v.dot(n))
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-(uv.dot(n))).min(1.0);
    let out_perp = (uv + (n * cos_theta)) * etai_over_etat;
    let out_parallel = n*(-((1.0-out_perp.length_squared()).abs()).sqrt());
    out_perp+out_parallel
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    //Schhhllliiickkkk
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}

pub struct Lambertian {
    pub albedo : Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
       let mut scatter_direction = hit.normal + hit.normal.random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

       Some((Ray{origin: hit.point, dir: scatter_direction}, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut reflected = reflect(ray.dir, hit.normal);
        reflected = reflected.unit_vector() + (reflected.random_unit_vector() * self.fuzz);
        Some((Ray{origin: hit.point, dir: reflected}, self.albedo))
    }
}

pub struct Dialectric {
    pub refraction_index: f64
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let ri;
        if hit.front_face {
            ri = 1.0 / self.refraction_index;
        } else {
            ri = self.refraction_index;
        }
        let unit_direction = ray.dir.unit_vector();
        let cos_theta = -unit_direction.dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction;

        if cannot_refract || (reflectance(cos_theta, ri) > random()){
            direction = reflect(unit_direction, hit.normal);
        } else {
            direction = refract(unit_direction, hit.normal, ri);
        }
        Some((Ray{origin: hit.point, dir: direction}, Vec3{x:1.0, y:1.0, z:1.0}))
    }
}