#![allow(non_snake_case)]

mod color;
mod hittable;
mod ray;
mod vec3;
mod prelude;
mod interval;
mod camera;
mod material;

use prelude::*;
use crate::hittable::{HittableList, Sphere};
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Dielectric};

fn main() {
    let mut world = HittableList::new();

    let material_ground = make_shared!(Lambertian, Color::new(0.8, 0.8, 0.0));
    let material_center = make_shared!(Lambertian, Color::new(0.1, 0.2, 0.5));
    let material_left   = make_shared!(Dielectric, 1.50);
    let material_bubble = make_shared!(Dielectric, 1.00 / 1.50);
    let material_right  = make_shared!(Metal, Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(make_shared!(Sphere, Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(make_shared!(Sphere, Point3::new(0.0,    0.0, -1.2),   0.5, material_center));
    world.add(make_shared!(Sphere, Point3::new(-1.0,   0.0, -1.0),   0.5, material_left));
    world.add(make_shared!(Sphere, Point3::new(-1.0,   0.0, -1.0),   0.4, material_bubble));
    world.add(make_shared!(Sphere, Point3::new(1.0,    0.0, -1.0),   0.5, material_right));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.v_fov = 20.0;
    cam.look_from = Point3::new(-2.0, 2.0, 1.0);
    cam.look_at = Point3::new(0.0, 0.0, -1.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.apature_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}