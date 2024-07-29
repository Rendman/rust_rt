pub mod ray;
pub mod hit;
pub mod sphere;
pub mod camera;
pub mod material;

use std::time::SystemTime;

use material::{Dialectric, Lambertian, Metal};
use nalgebra::Vector3;
use rand::Rng;
use hit::Hittable;
use sphere::Sphere;
use camera::Camera;

fn main() {
    let mut world : Vec<Box<dyn Hittable>> = Vec::new();

    let mut rng = rand::thread_rng();

    let ground_material = Lambertian {albedo: Vector3::new( 0.5, 0.5, 0.5)};
    world.push(Box::new(Sphere {center: Vector3::new(0.0, -1000.0, 0.0), radius: 1000.0, mat: ground_material}));



    for a in -11..11 {
        for b in -11..11 {
            let choose_mat:f64 = rng.gen();
            let aa = a as f64;
            let bb = b as f64;
            let center = Vector3::new(rng.gen::<f64>() * 0.9 + aa, 0.2, rng.gen::<f64>() * 0.9 + bb);

            if (center - (Vector3::new(4.0, 0.2, 0.0))).magnitude() > 0.9 {
                 if choose_mat < 0.8 {
                    let t_albedo : Vector3<f64> = Vector3::new(rng.gen(), rng.gen(), rng.gen());
                    let y_albedo : Vector3<f64> = Vector3::new(rng.gen(), rng.gen(), rng.gen());
                    let albedo : Vector3<f64> = t_albedo.component_mul(&y_albedo);
                    world.push(Box::new(Sphere {center: center, radius: 0.2, mat: Lambertian{albedo: albedo}}));
                 } else if choose_mat < 0.95 {
                    let albedo = Vector3::new(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0));
                    let fuzz: f64 = rng.gen_range(0.0..0.5);
                    world.push(Box::new(Sphere {center: center, radius: 0.2, mat: Metal{albedo: albedo, fuzz: fuzz}}));                                          
                 } else {
                    world.push(Box::new(Sphere {center: center, radius: 0.2, mat: Dialectric{refraction_index: 1.5}}));                                          
                 }
            }
        } 
    }

    world.push(Box::new(Sphere {center: Vector3::new(0.0, 1.0, 0.0), radius: 1.0, mat: Dialectric{refraction_index: 1.5}}));
    world.push(Box::new(Sphere {center: Vector3::new(-4.0, 1.0, 0.0), radius: 1.0, mat: Lambertian{albedo: Vector3::new(0.4, 0.2, 0.1)}}));

    world.push(Box::new(Sphere {center: Vector3::new(4.0,1.0, 0.0), radius: 1.0, mat: Metal{albedo: Vector3::new(0.7, 0.6, 0.5), fuzz: 0.0}}));


    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(13.0, 2.0, 3.0);
    cam.lookat = Vector3::new(0.0, 0.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let ctime = SystemTime::now();
    println! ("rust-rt starting...");

    cam.render(&world);

    match ctime.elapsed() {
        Ok(elapsed) => {
            println!("Done in {} secs", elapsed.as_secs());
        }
        Err(_) => {}
    }
}