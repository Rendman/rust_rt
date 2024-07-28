pub mod vec3;
pub mod color;
pub mod ray;
pub mod hit;
pub mod sphere;
pub mod camera;
pub mod material;

use std::time::SystemTime;

use material::{Dialectric, Lambertian, Metal};
use rand::Rng;
use vec3::Vec3;
use hit::Hittable;
use sphere::Sphere;
use camera::Camera;

fn main() {
    let mut world : Vec<Box<dyn Hittable>> = Vec::new();

    let mut rng = rand::thread_rng();

    let ground_material = Lambertian {albedo: Vec3{x: 0.5, y: 0.5, z: 0.5}};
    //world.push(Box::new(Sphere {center: Vec3{x:0.0, y: -1000.0, z: 0.0}, radius: 1000.0, mat: Rc::new(ground_material)}));
    world.push(Box::new(Sphere {center: Vec3{x:0.0, y: -1000.0, z: 0.0}, radius: 1000.0, mat: ground_material}));



    for a in -11..11 {
        for b in -11..11 {
            let choose_mat:f64 = rng.gen();
            let center = Vec3{x: rng.gen::<f64>() * 0.9 + a as f64, y: 0.2, z: rng.gen::<f64>() * 0.9 + b as f64};

            if (center - (Vec3{x: 4.0, y: 0.2, z: 0.0})).length() > 0.9 {
                 if choose_mat < 0.8 {
                    let t_albedo = Vec3::random();
                    let y_albedo = Vec3::random();
                    let albedo = t_albedo * y_albedo;
                    world.push(Box::new(Sphere {center: center, radius: 0.2, mat: Lambertian{albedo: albedo}}));
                 } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_bounds(0.5..1.0);
                    let fuzz: f64 = rng.gen_range(0.0..0.5);
                    world.push(Box::new(Sphere {center: center, radius: 0.2, mat: Metal{albedo: albedo, fuzz: fuzz}}));                                          
                 } else {
                    world.push(Box::new(Sphere {center: center, radius: 0.2, mat: Dialectric{refraction_index: 1.5}}));                                          
                 }
            }
        } 
    }

    world.push(Box::new(Sphere {center: Vec3{x: 0.0, y: 1.0, z: 0.0}, radius: 1.0, mat: Dialectric{refraction_index: 1.5}}));
    world.push(Box::new(Sphere {center: Vec3{x: -4.0, y: 1.0, z: 0.0}, radius: 1.0, mat: Lambertian{albedo: Vec3{x: 0.4, y: 0.2, z: 0.1}}}));
    world.push(Box::new(Sphere {center: Vec3{x: 4.0, y: 1.0, z: 0.0}, radius: 1.0, mat: Metal{albedo: Vec3{x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}}));


    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Vec3{x: 13.0, y: 2.0, z: 3.0};
    cam.lookat = Vec3{x: 0.0, y: 0.0, z: 0.0};
    cam.vup = Vec3 {x: 0.0, y: 1.0, z: 0.0};

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