#![allow(non_snake_case)]
#![allow(dead_code)]

mod color;
mod hittable;
mod ray;
mod vec3;
mod prelude;
mod interval;
mod camera;
mod material;
mod aabb;
mod texture;

use crate::prelude::*;
use crate::hittable::{HittableList, Sphere};
use crate::camera::Camera;
use crate::texture::{Texture, SolidColor, CheckerTexture};
use crate::material::{Material, Lambertian, Metal, Dielectric};

fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::from_texture(checker.into())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_f64();
            let center = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = material::Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0.0, random_f64_range(0.0, 0.35), 0.0);
                    world.add(Sphere::new_moving(center, center2,0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = material::Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = material::Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = material::Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = material::Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.v_fov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.apature_angle = 0.3;
    cam.focus_dist = 10.0;

    // Build BVH from world
    let world = world.to_bvh();
    cam.render(world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    world.add(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, Lambertian::from_texture(checker.clone().into())));
    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, Lambertian::from_texture(checker.into())));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.v_fov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.apature_angle = 0.0;

    cam.render(world);
}

fn main() {
    checkered_spheres();
}