Raytracer in rust

https://raytracing.github.io/books/RayTracingInOneWeekend.html

cam.image_width = 1200;
cam.samples_per_pixel = 500;
cam.max_depth = 50;

I1.0: 6344 secs (105.7 mins)
I1.1: 6264 secs
I1.2: 6496 secs

Iteration 2:
The Hittable implementation for Sphere uses .clone() on the Material field in the HitRecord. Does this deep copy cause a performance issue? Refactoring to eliminate the clone and just pass references to the original material.

I2.0: 6600 secs, lol

Iteration 3.... IDK