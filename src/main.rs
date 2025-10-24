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

use std::sync::Arc;

use crate::prelude::*;
use crate::camera::Camera;
use crate::hittable::{HittableList, Sphere};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::texture::{CheckerTexture, ImageTexture, SolidColor, Texture};

fn bouncing_spheres() {
    let mut world = HittableList::new();
    // TODO: Simplify texture construction
    // TODO: Make constructors use Arc references to allow sharing textures
    let checker_texture: Arc<Texture> = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)).into();
    let checker_material: Arc<Material> = Lambertian::from_texture(checker_texture).into();
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, checker_material));

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
                    let sphere_material: Arc<Material> = Lambertian::new(albedo).into();
                    let center2 = center + Vec3::new(0.0, random_f64_range(0.0, 0.35), 0.0);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material: Arc<Material> = Metal::new(albedo, fuzz).into();
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material: Arc<Material> = Dielectric::new(1.5).into();
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1: Arc<Material> = Dielectric::new(1.5).into();
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2: Arc<Material> = Lambertian::new(Color::new(0.4, 0.2, 0.1)).into();
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3: Arc<Material> = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0).into();
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

    let world = world.to_bvh(); // Build BVH from world
    cam.render(world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker_texture: Arc<Texture> = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)).into();
    let checker_material: Arc<Material> = Lambertian::from_texture(checker_texture).into();
    world.add(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, checker_material.clone()));
    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, checker_material.clone()));

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

    let world = world.to_bvh(); // Build BVH from world
    cam.render(world);
}

fn earth() {
    let earth_texture: Arc<Texture> = ImageTexture::from_file("earthmap.jpg").into();
    let earth_material: Arc<Material> = Lambertian::from_texture(earth_texture).into();
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_material);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.v_fov = 20.0;
    cam.look_from = Point3::new(0.0, 0.0, 12.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.apature_angle = 0.0;

    cam.render(globe);
}

fn solar_system() {
    let mut world = HittableList::new();

    // Sun (bright texture; Lambertian here since no emissive material is in use)
    let sun_tex: Arc<Texture> = ImageTexture::from_file("2k_sun.jpg").into();
    let sun_mat: Arc<Material> = Lambertian::from_texture(sun_tex).into();
    world.add(Sphere::new(Point3::new(-6.0, 0.0, 0.0), 2.2, sun_mat));

    // Mercury
    let mercury_tex: Arc<Texture> = ImageTexture::from_file("2k_mercury.jpg").into();
    let mercury_mat: Arc<Material> = Lambertian::from_texture(mercury_tex).into();
    world.add(Sphere::new(Point3::new(-3.2, 0.05, -0.2), 0.15, mercury_mat));

    // Venus
    let venus_tex: Arc<Texture> = ImageTexture::from_file("2k_venus_surface.jpg").into();
    let venus_mat: Arc<Material> = Lambertian::from_texture(venus_tex).into();
    world.add(Sphere::new(Point3::new(-2.3, 0.07, 0.4), 0.24, venus_mat));

    // Earth
    let earth_tex: Arc<Texture> = ImageTexture::from_file("2k_earth_daymap.jpg").into();
    let earth_mat: Arc<Material> = Lambertian::from_texture(earth_tex).into();
    world.add(Sphere::new(Point3::new(-1.4, 0.09, -0.35), 0.26, earth_mat));

    // Mars
    let mars_tex: Arc<Texture> = ImageTexture::from_file("2k_mars.jpg").into();
    let mars_mat: Arc<Material> = Lambertian::from_texture(mars_tex).into();
    world.add(Sphere::new(Point3::new(-0.4, 0.06, 0.2), 0.18, mars_mat));

    // Jupiter
    let jupiter_tex: Arc<Texture> = ImageTexture::from_file("2k_jupiter.jpg").into();
    let jupiter_mat: Arc<Material> = Lambertian::from_texture(jupiter_tex).into();
    world.add(Sphere::new(Point3::new(2.0, 0.2, 0.1), 0.95, jupiter_mat));

    // Saturn (rings not modeled; just a textured sphere)
    let saturn_tex: Arc<Texture> = ImageTexture::from_file("2k_saturn.jpg").into();
    let saturn_mat: Arc<Material> = Lambertian::from_texture(saturn_tex).into();
    world.add(Sphere::new(Point3::new(4.6, 0.16, -0.25), 0.8, saturn_mat));

    // Uranus
    let uranus_tex: Arc<Texture> = ImageTexture::from_file("2k_uranus.jpg").into();
    let uranus_mat: Arc<Material> = Lambertian::from_texture(uranus_tex).into();
    world.add(Sphere::new(Point3::new(6.7, 0.12, 0.35), 0.55, uranus_mat));

    // Neptune
    let neptune_tex: Arc<Texture> = ImageTexture::from_file("2k_neptune.jpg").into();
    let neptune_mat: Arc<Material> = Lambertian::from_texture(neptune_tex).into();
    world.add(Sphere::new(Point3::new(8.6, 0.1, -0.1), 0.52, neptune_mat));

    // Stars
    let star_texture: Arc<Texture> = ImageTexture::from_file("8k_stars_milky_way.jpg").into();
    let star_material: Arc<Material> = Lambertian::from_texture(star_texture).into();
    world.add(Sphere::new(Point3::new(0.0, -30.0, -50.0), 50.0, star_material));

    // Camera tuned to frame the whole arc
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.v_fov = 35.0;
    // Raise the camera and tilt it downward for an aerial view
    cam.look_from = Point3::new(0.0, 10.0, 20.0);
    cam.look_at = Point3::new(1.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.apature_angle = 0.0;

    let world = world.to_bvh();
    cam.render(world);
}

fn main() {
    match 4 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => solar_system(),
        _ => println!("No scene selected."),
    }
}
