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
mod onb;
mod pdf;
mod ply;

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
    cam.render(world, None);
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
    cam.render(world, None);
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
    cam.render(world, None);
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
    cam.render(world, None);
}

fn simple_light() {
    let mut world = HittableList::new();

    let perlin_texture = Texture::noise(4.0);
    let perlin_material = Material::lambertian_tex(perlin_texture);
    world.add(Sphere::new(&Point3::new(0.0, -1000.0, 0.0), 1000.0, perlin_material.clone()));
    world.add(Sphere::new(&Point3::new(0.0, 2.0, 0.0), 2.0, perlin_material));

    let light_material = Material::diffuse_light(Color::new(4.0, 4.0, 4.0));
    world.add(Sphere::new(&Point3::new(0.0, 7.0, 0.0), 2.0, light_material.clone()));
    world.add(Quad::new(&Point3::new(3.0, 1.0, -2.0), &Vec3::new(2.0, 0.0, 0.0), &Vec3::new(0.0, 2.0, 0.0), light_material));

    let mut cam = Camera::default();

    cam.background = Color::zero();

    cam.v_fov = 20.0;
    cam.look_from = Point3::new(26.0, 3.0, 6.0);
    cam.look_at = Point3::new(0.0, 2.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.aperture_angle = 0.0;

    let world = world.into_bvh();
    cam.render(world, None);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));

    world.add(Quad::new(&Point3::new(555.0, 0.0, 0.0), &Vec3::new(0.0, 555.0, 0.0), &Vec3::new(0.0, 0.0, 555.0), green));
    world.add(Quad::new(&Point3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 555.0, 0.0), &Vec3::new(0.0, 0.0, 555.0), red));
    world.add(Quad::new(&Point3::new(0.0, 0.0, 0.0), &Vec3::new(555.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 555.0), white.clone()));
    world.add(Quad::new(&Point3::new(555.0, 555.0, 555.0), &Vec3::new(-555.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -555.0), white.clone()));
    world.add(Quad::new(&Point3::new(0.0, 0.0, 555.0), &Vec3::new(555.0, 0.0, 0.0), &Vec3::new(0.0, 555.0, 0.0), white.clone()));

    // Box 1
    let aluminum = Material::metal(Color::new(0.8, 0.85, 0.88), 0.0);
    let box1 = Cuboid::from_center_rotate_y(
        &Point3::new(365.0, 330.0/2.0, 325.0), 
        &Vec3::new(165.0, 330.0, 165.0), 
        15.0,
        aluminum.clone(),
    );
    world.add(box1.clone());

    // Glass sphere
    let glass_mat = Material::dielectric(1.5);
    let glass_sphere = Sphere::new(&Point3::new(190.0, 90.0, 190.0), 90.0, glass_mat);
    world.add(glass_sphere.clone());

    // Light quad
    let light_mat = Material::diffuse_light(Color::new(15.0, 15.0, 15.0));
    let light_quad = Quad::new(&Point3::new(343.0, 554.0, 332.0), &Vec3::new(-130.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, -105.0), light_mat);
    world.add(light_quad.clone());

    // Setup for importance sampling
    let mut sampling_list = HittableList::new();
    sampling_list.add(light_quad);
    sampling_list.add(glass_sphere);
    sampling_list.add(box1);
    let sampling_list = Arc::new(Hittable::HittableList(sampling_list));

    let mut cam = Camera::default();
    cam.scene_name = "cornell_box".to_string();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.v_fov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.aperture_angle = 0.0;

    let world = world.into_bvh();
    cam.render(world, Some(sampling_list));
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    // Materials
    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(Color::new(7.0, 7.0, 7.0)); // larger, dimmer light

    // Cornell walls
    world.add(Quad::new(&Point3::new(555.0, 0.0, 0.0), &Vec3::new(0.0, 555.0, 0.0), &Vec3::new(0.0, 0.0, 555.0), green));
    world.add(Quad::new(&Point3::new(0.0, 0.0, 0.0),   &Vec3::new(0.0, 555.0, 0.0), &Vec3::new(0.0, 0.0, 555.0), red));
    // Big area light on ceiling (normal points downward)
    world.add(Quad::new(&Point3::new(113.0, 554.0, 127.0), &Vec3::new(330.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 305.0), light));
    world.add(Quad::new(&Point3::new(0.0,   555.0, 0.0),   &Vec3::new(555.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 555.0), white.clone()));
    world.add(Quad::new(&Point3::new(0.0,   0.0,   0.0),   &Vec3::new(555.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 555.0), white.clone()));
    world.add(Quad::new(&Point3::new(0.0,   0.0, 555.0),   &Vec3::new(555.0, 0.0, 0.0), &Vec3::new(0.0, 555.0, 0.0), white.clone()));

    // Boundary boxes (white), then wrapped in ConstantMedium volumes
    let box1 = Cuboid::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = Hittable::translate(
        Hittable::rotate_y(box1, 15.0),
        Vec3::new(265.0, 0.0, 295.0),
    );

    let box2 = Cuboid::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = Hittable::translate(
        Hittable::rotate_y(box2, -18.0),
        Vec3::new(130.0, 0.0, 65.0),
    );

    // Fog volumes: dark smoke and light fog
    let smoke_black = ConstantMedium::new(Arc::new(box1), 0.01, &Color::new(0.0, 0.0, 0.0));
    let smoke_white = ConstantMedium::new(Arc::new(box2), 0.01, &Color::new(1.0, 1.0, 1.0));
    world.add(smoke_black);
    world.add(smoke_white);

    // Camera
    let mut cam = Camera::default();
    cam.scene_name = "cornell_smoke".to_string();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 4000;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.v_fov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);
    cam.aperture_angle = 0.0;

    let world = world.into_bvh();
    cam.render(world, None);
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    let mut boxes1 = HittableList::new();
    let ground = Material::lambertian(Color::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Cuboid::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();

    // Add the ground boxes as a BVH
    world.add(boxes1.into_bvh());

    // Large area light
    let light = Material::diffuse_light(Color::new(7.0, 7.0, 7.0));
    let light_quad = Quad::new(
        &Point3::new(123.0, 554.0, 147.0),
        &Vec3::new(300.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 265.0),
        light,
    );
    world.add(light_quad.clone());

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Material::lambertian(Color::new(0.7, 0.3, 0.1));
    world.add(Sphere::new_moving(&center1, &center2, 50.0, sphere_material));

    // Glass sphere
    world.add(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        Material::dielectric(1.5),
    ));

    // Metal sphere
    world.add(Sphere::new(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        Material::metal(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    // Blue subsurface sphere (glass sphere with blue fog inside)
    let boundary = Arc::new(Hittable::from(Sphere::new(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        Material::dielectric(1.5),
    )));
    world.add(boundary.as_ref().clone());
    world.add(ConstantMedium::new(
        boundary.clone(),
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    ));

    // Thin mist covering everything
    let boundary = Arc::new(Hittable::from(Sphere::new(
        &Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Material::dielectric(1.5),
    )));
    world.add(ConstantMedium::new(
        boundary,
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    ));

    // Earth sphere
    let earth_material = Material::lambertian_tex(Texture::image("earthmap.jpg"));
    world.add(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    ));

    // Perlin noise sphere
    let perlin_texture = Texture::noise(0.2);
    world.add(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        Material::lambertian_tex(perlin_texture),
    ));

    // Box of small white spheres
    let mut boxes2 = HittableList::new();
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Sphere::new(
            &Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    world.add(Hittable::translate(
        Hittable::rotate_y(boxes2.into_bvh(), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.v_fov = 40.0;
    cam.look_from = Point3::new(478.0, 278.0, -600.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.aperture_angle = 0.0;

    let world = world.into_bvh();
    let light_target = Arc::new(Hittable::Quad(light_quad));
    cam.render(world, Some(light_target));
}

fn pyramid() {
    let mut world = HittableList::new();

    // Ground plane
    let ground_checker = Texture::checker(0.5, Color::new(0.1, 0.1, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_mat = Material::lambertian_tex(ground_checker);
    world.add(Quad::new(&Point3::new(-10.0, 0.0, -10.0), &Vec3::new(20.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 20.0), ground_mat));

    // Pyramid: apex at top, square base below
    let apex = Point3::new(0.0, 2.0, 0.0);
    let base_a = Point3::new(-1.0, 0.1, -1.0);
    let base_b = Point3::new(1.0, 0.1, -1.0);
    let base_c = Point3::new(1.0, 0.1, 1.0);
    let base_d = Point3::new(-1.0, 0.1, 1.0);

    // Mixed materials for more visual interest
    let mat_red_metal = Material::metal(Color::new(1.0, 0.2, 0.2), 0.3);
    let mat_green = Material::lambertian(Color::new(0.2, 1.0, 0.2));
    let mat_blue_metal = Material::metal(Color::new(0.2, 0.2, 1.0), 0.2);
    let mat_yellow = Material::lambertian(Color::new(1.0, 1.0, 0.2));
    let mat_base = Material::metal(Color::new(0.9, 0.9, 0.95), 0.1);

    // Four triangular faces
    world.add(Triangle::new(&apex, &base_a, &base_b, mat_red_metal));      // Front face (red metal)
    world.add(Triangle::new(&apex, &base_b, &base_c, mat_green));          // Right face (green)
    world.add(Triangle::new(&apex, &base_c, &base_d, mat_blue_metal));     // Back face (blue metal)
    world.add(Triangle::new(&apex, &base_d, &base_a, mat_yellow));         // Left face (yellow)

    // Base (two triangles)
    world.add(Triangle::new(&base_a, &base_b, &base_c, mat_base.clone()));
    world.add(Triangle::new(&base_a, &base_c, &base_d, mat_base));

    // Add a glass sphere for reflection/refraction interest
    world.add(Sphere::new(&Point3::new(2.5, 1.0, 2.5), 0.8, Material::dielectric(1.5)));

    // Add a metal sphere
    world.add(Sphere::new(&Point3::new(-2.5, 0.7, -2.5), 0.7, Material::metal(Color::new(1.0, 0.84, 0.0), 0.2)));

    // Rotate pyramid for visibility
    let pyramid_bvh = world.into_bvh();
    let pyramid = Hittable::rotate_y(pyramid_bvh, 35.0);

    // Light quad above the scene
    let light_mat = Material::diffuse_light(Color::new(2.0, 2.0, 2.0));
    let mut scene = HittableList::from_hittable(pyramid);
    scene.add(Quad::new(&Point3::new(-5.0, 5.0, -5.0), &Vec3::new(10.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 10.0), light_mat));

    let mut cam = Camera::default();
    cam.aspect_ratio = 1.0;
    cam.image_width = 1024;
    cam.samples_per_pixel = 3000;
    cam.max_depth = 50;
    cam.background = Color::new(0.15, 0.15, 0.2);

    cam.v_fov = 45.0;
    cam.look_from = Point3::new(4.5, 3.0, 4.5);
    cam.look_at = Point3::new(0.0, 1.2, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);
    cam.aperture_angle = 0.0;

    let final_world = scene.into_bvh();
    cam.render(final_world, None);
}

fn ply_model_scene() {
    // Load geometry from file
    let mut data = ply::load("models/dragon.ply").expect("load PLY");

    let scale = 8.0;
    for p in &mut data.positions {
        *p = *p * scale;
    }
    
    // Build TriangleMesh
    let mat1 = Material::lambertian(Color::new(0.7, 0.7, 0.7));
    let mat2 = Material::metal(Color::new(0.8, 0.8, 0.9), 0.1);
    let mat3 = Material::dielectric(1.5);
    let mesh = TriangleMesh::new(data.positions, data.indices, mat1);

    // Add to world
    let mesh = Hittable::rotate_y_translate(mesh, 180.0-35.0, Vec3::new(-0.1, -0.4, -1.0));

    let mut world = HittableList::new();
    world.add(mesh);

    let ground = Material::lambertian(Color::new(0.4, 0.4, 0.4));
    world.add(Quad::new(
        &Point3::new(-10.0, 0.0, -10.0),
        &Vec3::new(20.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 20.0),
        ground));

    let light = Material::diffuse_light(Color::new(7.0, 7.0, 7.0));
    let light_quad = Quad::new(
        &Point3::new(-2.0, 3.99, -2.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 4.0), 
        light,
    );
    world.add(light_quad.clone());

    let mut cam = Camera::default();
    cam.scene_name = "ply_model".to_string();

    cam.aspect_ratio = 1.0;
    cam.image_width = 1024;
    cam.samples_per_pixel = 500;
    cam.max_depth = 20;
    cam.background = Color::new(0.02, 0.02, 0.03);

    cam.v_fov = 20.0;
    cam.look_from = Point3::new(0.0, 1.5, 4.0);
    cam.look_at = Point3::new(0.0, 0.75, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);
    cam.aperture_angle = 0.0;

    let world = world.into_bvh();
    let sampling_target = Arc::new(Hittable::Quad(light_quad));
    cam.render(world, Some(sampling_target));
}

fn main() {
    match 11 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => perlin_spheres(),
        4 => quads(),
        5 => simple_light(),
        6 => cornell_box(),
        7 => cornell_smoke(),
        8 => final_scene(800, 10000, 40),  // High quality
        9 => final_scene(400, 250, 4),     // Quick preview
        10 => pyramid(),
        11 => ply_model_scene(),
        _ => println!("No scene selected."),
    }
}
// TODO: Have each scene as a separate file and use command line args to select which one to render. 