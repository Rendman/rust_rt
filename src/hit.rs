use std::ops::Range;
use nalgebra::Vector3;

use crate::{material::Material, ray::Ray};

pub struct HitRecord<'a> {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    //pub mat: Rc<dyn Material>
    pub mat: &'a dyn Material
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Range<f64>) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.end;
        let mut hit_anything: Option<HitRecord> = None;

        for object in self.iter() {
            
            if let Some(hit) = object.hit(r, ray_t.start..closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything  
    }
}