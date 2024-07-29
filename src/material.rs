use nalgebra::Vector3;
use rand::{random, Rng};

use crate::{hit::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - (n * 2.0 * v.dot(&n))
}

pub fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64>   {
    let cos_theta = (-(uv.dot(&n))).min(1.0);
    let out_perp = (uv + (n * cos_theta)) * etai_over_etat;
    let out_parallel = n*(-((1.0-out_perp.norm_squared()).abs()).sqrt());
    out_perp+out_parallel
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    //Schhhllliiickkkk
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}

pub fn near_zero(vec: Vector3<f64>) -> bool {
    return f64::abs(vec.x) < f64::EPSILON && f64::abs(vec.y) < f64::EPSILON && f64::abs(vec.z) < f64::EPSILON;
}

pub fn random_unit_vector() -> Vector3<f64> {
    let mut rnd = rand::thread_rng();
    
    loop {
        //let p = Vector3::random_bounds(-1.0..1.0);
        let p = Vector3::new(rnd.gen_range(-1.0..1.0), rnd.gen_range(-1.0..1.0), rnd.gen_range(-1.0..1.0));
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub struct Lambertian {
    pub albedo : Vector3<f64>
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
       let mut scatter_direction = hit.normal + random_unit_vector();
        if near_zero(scatter_direction) {
            scatter_direction = hit.normal;
        }

       Some((Ray{origin: hit.point, dir: scatter_direction}, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let mut reflected = reflect(ray.dir, hit.normal);
        reflected = reflected.normalize() + (random_unit_vector() * self.fuzz);
        Some((Ray{origin: hit.point, dir: reflected}, self.albedo))
    }
}

pub struct Dialectric {
    pub refraction_index: f64
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let ri;
        if hit.front_face {
            ri = 1.0 / self.refraction_index;
        } else {
            ri = self.refraction_index;
        }
        let unit_direction = ray.dir.normalize();
        let cos_theta = -unit_direction.dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction;

        if cannot_refract || (reflectance(cos_theta, ri) > random()){
            direction = reflect(unit_direction, hit.normal);
        } else {
            direction = refract(unit_direction, hit.normal, ri);
        }
        Some((Ray{origin: hit.point, dir: direction}, Vector3::new(1.0, 1.0, 1.0)))
    }
}