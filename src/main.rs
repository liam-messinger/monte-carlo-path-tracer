#![allow(non_snake_case)]
#![allow(dead_code)]

mod camera;
mod color;
mod hittable;
mod image_data;
mod interval;
mod material;
mod prelude;
mod ray;
mod texture;
mod vec3;
mod noise;

use std::sync::Arc;

use crate::prelude::*;
use crate::camera::Camera;
use crate::hittable::*;
use crate::material::*;
use crate::texture::*;

fn bouncing_spheres() {
    let mut world = HittableList::new();
    let checker_texture = Texture::checker(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let checker_material = Material::lambertian_tex(checker_texture);
    world.add(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, checker_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::lambertian(albedo);
                    let center2 = center + Vec3::new(0.0, random_f64_range(0.0, 0.35), 0.0);
                    world.add(Sphere::new_moving(&center, &center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Material::metal(albedo, fuzz);
                    world.add(Sphere::new(&center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Material::dielectric(1.5);
                    world.add(Sphere::new(&center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::dielectric(1.5);
    world.add(Sphere::new(&Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(&Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(&Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let mut cam = Camera::high_quality_default();

    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.aperture_angle = 0.3;
    cam.focus_dist = 10.0;

    let world = world.into_bvh(); // Build BVH from world
    cam.render(world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker_texture = Texture::checker(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let checker_material = Material::lambertian_tex(checker_texture);
    world.add(Sphere::new(&Point3::new(0.0, -10.0, 0.0), 10.0, checker_material.clone()));
    world.add(Sphere::new(&Point3::new(0.0, 10.0, 0.0), 10.0, checker_material.clone()));

    let mut cam = Camera::high_quality_default();

    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    let world = world.into_bvh(); // Build BVH from world
    cam.render(world);
}

fn earth() {
    let earth_material = Material::lambertian_tex(Texture::image("earthmap.jpg"));
    let globe = Sphere::new(&Point3::new(0.0, 0.0, 0.0), 2.0, earth_material);

    let mut cam = Camera::high_quality_default();

    cam.look_from = Point3::new(0.0, 0.0, 12.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.render(globe);
}

fn solar_system() {
    let mut world = HittableList::new();

    // Sun
    let sun_mat = Material::lambertian_tex(Texture::image("2k_sun.jpg"));
    world.add(Sphere::new(&Point3::new(-6.0, 0.0, 0.0), 2.2, sun_mat));

    // Mercury
    let mercury_mat = Material::lambertian_tex(Texture::image("2k_mercury.jpg"));
    world.add(Sphere::new(&Point3::new(-3.2, 0.05, -0.2), 0.15, mercury_mat));

    // Venus
    let venus_mat = Material::lambertian_tex(Texture::image("2k_venus_surface.jpg"));
    world.add(Sphere::new(&Point3::new(-2.3, 0.07, 0.4), 0.24, venus_mat));

    // Earth
    let earth_mat = Material::lambertian_tex(Texture::image("2k_earth_daymap.jpg"));
    world.add(Sphere::new(&Point3::new(-1.4, 0.09, -0.35), 0.26, earth_mat));

    // Mars
    let mars_mat = Material::lambertian_tex(Texture::image("2k_mars.jpg"));
    world.add(Sphere::new(&Point3::new(-0.4, 0.06, 0.2), 0.18, mars_mat));

    // Jupiter
    let jupiter_mat = Material::lambertian_tex(Texture::image("2k_jupiter.jpg"));
    world.add(Sphere::new(&Point3::new(2.0, 0.2, 0.1), 0.95, jupiter_mat));

    // Saturn (rings not modeled; just a textured sphere)
    let saturn_mat = Material::lambertian_tex(Texture::image("2k_saturn.jpg"));
    world.add(Sphere::new(&Point3::new(4.6, 0.16, -0.25), 0.8, saturn_mat));

    // Uranus
    let uranus_mat = Material::lambertian_tex(Texture::image("2k_uranus.jpg"));
    world.add(Sphere::new(&Point3::new(6.7, 0.12, 0.35), 0.55, uranus_mat));

    // Neptune
    let neptune_mat = Material::lambertian_tex(Texture::image("2k_neptune.jpg"));
    world.add(Sphere::new(&Point3::new(8.6, 0.1, -0.1), 0.52, neptune_mat));

    // Stars
    let star_material = Material::lambertian_tex(Texture::image("8k_stars_milky_way.jpg"));
    world.add(Sphere::new(&Point3::new(0.0, -30.0, -50.0), 50.0, star_material));

    // Camera tuned to frame the whole arc
    let mut cam = Camera::high_quality_default();

    cam.v_fov = 35.0;
    // Raise the camera and tilt it downward for an aerial view
    cam.look_from = Point3::new(0.0, 10.0, 20.0);
    cam.look_at = Point3::new(1.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.aperture_angle = 0.0;

    let world = world.into_bvh();
    cam.render(world);
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let perlin_material = Material::lambertian_tex(Texture::noise(4.0));
    world.add(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, perlin_material.clone()));
    world.add(Sphere::new(&Point3::new(0.0, 2.0, 0.0), 2.0, perlin_material.clone()));

    let mut cam = Camera::high_quality_default();

    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    let world = world.into_bvh(); // Build BVH from world
    cam.render(world);
}

fn quads() {
    let mut world: HittableList = HittableList::new();

    // Materials
    let left_material = Material::lambertian(Color::new(1.0, 0.2, 0.2));
    let back_material = Material::lambertian(Color::new(0.2, 1.0, 0.2));
    let right_material = Material::metal(Color::new(0.8, 0.8, 0.9), 0.1);
    let upper_material = Material::lambertian(Color::new(0.2, 0.2, 1.0));
    let lower_material = Material::lambertian_tex(Texture::image("earthmap.jpg"));

    // Quads
    world.add(Quad::new(&Point3::new(-3.0, -2.0, 5.0), &Vec3::new(0.0, 0.0, -4.0), &Vec3::new(0.0, 4.0, 0.0), left_material));
    world.add(Quad::new(&Point3::new(-2.0, -2.0, 0.0), &Vec3::new(4.0, 0.0, 0.0), &Vec3::new(0.0, 4.0, 0.0), back_material));
    world.add(Quad::new(&Point3::new(3.0, -2.0, 1.0), &Vec3::new(0.0, 0.0, 4.0), &Vec3::new(0.0, 4.0, 0.0), right_material));
    world.add(Quad::new(&Point3::new(-2.0, 3.0, 1.0), &Vec3::new(4.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 4.0), upper_material));
    world.add(Quad::new(&Point3::new(-2.0, -3.0, 5.0), &Vec3::new(4.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -4.0), lower_material));

    // Middle glass sphere
    world.add(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0), 
        2.0, 
        Material::dielectric(2.5),
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 250;
    cam.max_depth = 50;

    cam.v_fov = 80.0;
    cam.look_from = Point3::new(0.0, 0.0, 9.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.aperture_angle = 0.0;

    let world = world.into_bvh(); // Build BVH from world
    cam.render(world);
}

fn main() {
    match 6 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => solar_system(),
        5 => perlin_spheres(),
        6 => quads(),
        _ => println!("No scene selected."),
    }
}
