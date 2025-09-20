#![allow(non_snake_case)]

// Internal modules
mod color;
mod hittable;
mod ray;
mod vec3;
mod prelude;
mod interval;
mod camera;

use prelude::*;
use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::camera::Camera;

use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}