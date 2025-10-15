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
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    let ground_material = make_shared!(Lambertian, Color::new(0.5, 0.5, 0.5));
    world.add(make_shared!(Sphere, Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_f64();
            let center = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = make_shared!(Lambertian, albedo);
                    world.add(make_shared!(Sphere, center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    sphere_material = make_shared!(Metal, albedo, fuzz);
                    world.add(make_shared!(Sphere, center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = make_shared!(Dielectric, 1.5);
                    world.add(make_shared!(Sphere, center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = make_shared!(Dielectric, 1.5);
    world.add(make_shared!(Sphere, Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = make_shared!(Lambertian, Color::new(0.4, 0.2, 0.1));
    world.add(make_shared!(Sphere, Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = make_shared!(Metal, Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(make_shared!(Sphere, Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.v_fov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.apature_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}