use core::f64;
use std::{fs::File, io::{self, BufWriter, Write}, path::Path};
use nalgebra::Vector3;
use rand::prelude::*;

use crate::{hit::Hittable, ray::Ray};


#[derive(Copy, Clone, Default)]
pub struct Camera {
    pub aspect_ratio : f64,
    pub samples_per_pixel: u32,
    pub image_width : u32,
    pub max_depth : u32,
    pub vfov: f64,
    pub lookfrom: Vector3<f64>,
    pub lookat: Vector3<f64>,
    pub vup: Vector3<f64>,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    pixel_sample_scale: f64,
    image_height : u32,
    center : Vector3<f64>,
    pixel_00loc: Vector3<f64>, 
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
}

// What does the camera need?  Initialization should be done via a new function.

pub struct Camera2 {
    aspect_ratio: f64,
    samples_per_pixel: u32,
    image_width: u32,
    max_depth: u32,
    
}


impl Camera {
    
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        if self.image_height < 1
        {
            self.image_height = 1;
        }

        self.center = self.lookfrom;

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
        
        let theta = self.vfov.to_radians();
        let h = (theta/2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        
        let viewport_width = viewport_height * ((self.image_width as f64)/self.image_height as f64);

        let w = (self.lookfrom - self.lookat).normalize();
        let u = (self.vup.cross(&w)).normalize();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - (w * self.focus_dist) - viewport_u/2.0 - viewport_v/2.0;

        self.pixel_00loc = (self.pixel_delta_u + self.pixel_delta_v) * 0.5 + viewport_upper_left;

        let defocus_radius = self.focus_dist * ((self.defocus_angle/2.0).to_radians()).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;

    }

    pub fn render(&mut self, world: &Vec<Box<dyn Hittable>>) {
        self.initialize();

        let mut image_data = Vec::new();

        // Create png writer stuff
        let path = Path::new(r"output.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        println!();
        println!("Rendering");
        let _ = io::stdout().flush();

        let mut encoder = png::Encoder::new(w, self.image_width, self.image_height);

        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                
                let mut pixel_color = Vector3::new(0.0,0.0,0.0);
                let mut idx = 0;
                while idx < self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, self.max_depth, &world);
                    idx = idx + 1;
                }
               
                let intensity = 0.0..0.999;

                pixel_color.x = pixel_color.x * self.pixel_sample_scale;
                pixel_color.y = pixel_color.y * self.pixel_sample_scale;
                pixel_color.z = pixel_color.z * self.pixel_sample_scale;

                let ir = (256.0 * f64::clamp(linear_to_gamma(pixel_color.x), intensity.start, intensity.end)) as u8;
                let ig = (256.0 * f64::clamp(linear_to_gamma(pixel_color.y), intensity.start, intensity.end)) as u8;
                let ib = (256.0 * f64::clamp(linear_to_gamma(pixel_color.z), intensity.start, intensity.end)) as u8;
            
                image_data.push(ir);
                image_data.push(ig);
                image_data.push(ib);
            }
            println!("{}.{}", j, self.image_height);
            let _ = io::stdout().flush();

        }
        writer.write_image_data(&image_data).unwrap();
        println!();
    }

    fn get_ray(self, i: u32, j: u32) -> Ray {
        let mut rng = thread_rng();
        let tx : f64 = rng.gen();
        let ty : f64 = rng.gen();

        let offset = Vector3::new(tx-0.5, ty-0.5, 0.0);
        
        let pixel_sample = self.pixel_00loc + (self.pixel_delta_u * (i as f64 + offset.x))
                                                  + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin;
        if self.defocus_angle <= 0.0 {
            ray_origin = self.center
        } else {
            let p = random_in_unit_disk();
            ray_origin = self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
        }
        let ray_direction = pixel_sample - ray_origin;

        Ray{origin: ray_origin, dir: ray_direction}
    }

    fn ray_color(self, ray: Ray, depth: u32, world: &Vec<Box<dyn Hittable>>) -> Vector3<f64> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        
        if let Some(hit) = world.hit(ray, 0.001..f64::MAX) {
            if let Some(at_sc) = hit.mat.scatter(&ray, &hit) {
                return Vector3::new(at_sc.1.x, at_sc.1.y, at_sc.1.z).component_mul(&self.ray_color(at_sc.0, depth-1, world))
                
            } else {
                return Vector3::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_direction = ray.dir.normalize();
            let a = 0.5*(unit_direction.y + 1.0);
            Vector3::new(1.0, 1.0, 1.0) * (1.0-a) + Vector3::new(0.5, 0.7, 1.0) * a
        }        
    }
}

fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = thread_rng(); 
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}