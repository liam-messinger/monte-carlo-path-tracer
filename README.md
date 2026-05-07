# Monte Carlo Path Tracer (Rust)

CPU Monte Carlo path tracer in Rust inspired by Ray Tracing in One Weekend series.

<img width="600" height="600" alt="zdcornell_box_600x600_10000spp_50depth_112m23s" src="https://github.com/user-attachments/assets/36dd1128-52eb-4219-b7d0-e2cb84c474b1" />

## Gallery

https://github.com/user-attachments/assets/a825c9b3-9471-4be1-8d90-095dd2a7dd68

<img width="768" height="768" alt="h_ply_model_1024x1024_500spp_20depth_7m39s" src="https://github.com/user-attachments/assets/79e92e19-db54-441d-a13b-acc8e86a9d43" />
<br>

![Final scene](images/output2.png)

## Features

- Geometry: spheres, quads, cuboids, triangles, triangle meshes (PLY loader, ASCII)
- Acceleration: BVH for worlds and meshes; SAH binned builder for meshes
- Materials: Lambertian (textured), metal, dielectric (glass), diffuse lights, isotropic (volumes)
- Textures: solid color, checker, image textures, Perlin noise
- Anti-aliasing: stratified sampling with configurable samples per pixel
- Importance Sampling: cosine-weighted, light-importance, mixture PDFs
- Volumetrics: constant-density media (fog/smoke)
- Camera: depth of field (aperture + focus distance), configurable FOV and orientation
- Parallelism: multi-threaded rendering with Rayon

## Quick Start

- Prerequisites: Rust + Cargo
- Run:

```bash
cargo run --release
or
cargo run -r
```

- Run (show normals):

```bash
cargo run --release --features normals
or
cargo run -r -F normals
```

- Switch scenes: tweak the `match` in [src/main.rs](src/main.rs) to pick a scene.

## Credits

- Inspired by Ray Tracing in One Weekend: https://raytracing.github.io/books/RayTracingInOneWeekend.html
