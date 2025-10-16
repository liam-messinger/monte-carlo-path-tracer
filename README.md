![heavy scene](./images/output.png)

---

## Heavy Scene Benchmarking For Improvements:
```
Dynamic Dispatch - 5:18
Static Dispatch - 4:26 (17% faster)
Inlining Functions - 4:07 (7% faster, 22% faster overall)
Removing hit method clone - 3:23 (18% faster, 36% faster overall)
Parallelism with Rayon - 0:50 (4.0x faster, 6.3x faster overall)
```
---
```rust
cam.aspect_ratio = 16.0 / 9.0;
cam.image_width = 1200;
cam.samples_per_pixel = 100;
cam.max_depth = 50;

cam.v_fov = 20.0;
cam.look_from = Point3::new(13.0, 2.0, 3.0);
cam.look_at = Point3::new(0.0, 0.0, 0.0);
cam.v_up = Vec3::new(0.0, 1.0, 0.0);

cam.apature_angle = 0.6;
cam.focus_dist = 10.0;
```