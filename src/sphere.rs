use std::{ops::Range, rc::Rc};

use crate::{hit::{HitRecord, Hittable}, ray::Ray, vec3::Vec3, material::Material};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Rc<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.dir.length_squared();
        let h = r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        
        if discriminant < 0.0 {
            return None
        }
        
        let sqrtd = discriminant.sqrt();
        
        let mut root = (h - sqrtd) / a;

        if root <= ray_t.start || ray_t.end <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.start || ray_t.end <= root {
                return None
            }
        }
      
        let pt = r.at(root);
        let outward_normal = (pt - self.center) / self.radius;
        let normal;

        let front_face = r.dir.dot(outward_normal) < 0.0;
        if front_face { 
            normal = outward_normal;
        } else {
            normal = -outward_normal;
        }

        Some(
            HitRecord {
                point: pt,
                normal: normal,
                t: root,
                front_face: front_face,
                mat: self.mat.clone()
            }
        )
    }

}

