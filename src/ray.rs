use nalgebra::Vector3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin : Vector3<f64>,
    pub dir: Vector3<f64>
}

impl Ray {
    pub fn at(&self, t:f64) -> Vector3<f64>{
        self.origin + self.dir * t
    }
}