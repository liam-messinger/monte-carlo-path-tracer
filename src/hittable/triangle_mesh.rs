use super::{AABB, HitRecord, Hittable};

use crate::material::Material;
use crate::prelude::{EPSILON, Interval, random_f64};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use std::sync::Arc;

// =====================================================================
// Build / Traversal constants
// =====================================================================

/// Maximum triangles allowed in a single BVH leaf. 2-8 is a common range;
const MAX_TRIANGLES_PER_LEAF: usize = 4;

/// Number of bins used by the binned SAH split search. 
/// More bins = better split quality at proportional build cost.
const SAH_BIN_COUNT: usize = 16;

/// SAH cost ratio. A leaf of N triangles costs `N * SAH_INTERSECT_COST`;
/// an internal node costs `SAH_TRAVERSE_COST + (P_l*N_l + P_r*N_r) * SAH_INTERSECT_COST`.
/// Low traverse cost favors deeper trees with smaller leaves; high traverse cost favors shallower trees with larger leaves.
const SAH_INTERSECT_COST: f64 = 1.0; 
const SAH_TRAVERSE_COST: f64 = 1.0;

/// Maximum depth of the BVH tree. Prevents infinite recursion in degenerate cases.
/// log2(1M / 4) ≈ 18, so 64 is a safe upper bound for typical scenes. 
const MAX_BVH_DEPTH: usize = 64;

// =====================================================================
// Hot-path intersection geometry
// =====================================================================

/// Minimal triangle representation, optimized for fast/cache-friendly ray-triangle intersection.
/// Stored in BVH-leaf order (not face order). Avoids recomputing edge vectors and cross products at each intersection.
/// Total size is 72 bytes (3x Point3 for vertices + 2x Vec3 for edges).
/// 
/// Normal is not stored here since it is only needed for scattering calculation after confirming a hit.
/// Saves 24 bytes per triangle, computed by cross product of e1 and e2.
#[derive(Clone, Copy)]
struct SimpleTriangle {
    p0: Point3,   // 24 B: vertex 0
    e1: Vec3,     // 24 B: edge from p0 to p1 (i.e., p1 - p0)
    e2: Vec3,     // 24 B: edge from p0 to p2 (i.e., p2 - p0)
}

// =====================================================================
// Flat triangle mesh BVH
// =====================================================================

/// Flat BVH node. When `triangle_count == 0` the node is internal (left child at index self+1,
/// right child index in `right_child_or_first_triangle`). Otherwise it is a leaf and
/// `right_child_or_first_triangle` is the starting index into `triangles` for this leaf.
#[derive(Clone)]
struct MeshBVHNode {
    bounding_box: AABB,                     // 48 B: bounding box of this node 
    right_child_or_first_triangle: u32,     // 4 B: if leaf, index of first triangle; if internal, index of right child
    triangle_count: u32,                    // 4 B: if leaf, number of triangles; if internal, 0
}

impl MeshBVHNode {
    /// Check if this node is a leaf (contains triangles) or an internal node (has children).
    #[inline] fn is_leaf(&self) -> bool { self.triangle_count > 0 }
}

// =====================================================================
// Geometry container
// =====================================================================

struct MeshGeometry {
    // Hot path buffers
    triangles: Vec<SimpleTriangle>,  // Contiguous array of triangles in BVH-leaf order
    bvh_nodes: Vec<MeshBVHNode>,     // Contiguous array of BVH nodes in left-child-first ordering

    // Sampling precomputation buffers
    face_areas: Vec<f64>,
    face_area_cdf: Vec<f64>,         // Defined as: face_area_cdf[i] = sum(face_areas[0..=i]) / total_area
    total_area: f64,

    // Optional buffers for future features (smooth normals / UV texture mapping)
    // Kept as Option so we can drop them after build for flat shading
    positions: Option<Vec<Point3>>,         // Original vertex positions
    face_indices: Option<Vec<[u32; 3]>>,    // Original face indices
    vertex_normals: Option<Vec<Vec3>>,      // Per-vertex normals for smooth shading
    vertex_uvs: Option<Vec<(f64, f64)>>,    // Per-vertex UV coordinates for texturing
}

/// Public mesh hittable with shared geometry and a single material.
#[derive(Clone)]
pub struct TriangleMesh {
    geometry: Arc<MeshGeometry>,
    material: Arc<Material>,
}

impl TriangleMesh {
    /// Build a `TriangleMesh` from a vertex buffer, triangle index buffer, and a single material.
    /// Performs the following steps:
    /// 1. Compute per-face bounds, centroids, and areas in original face order.
    /// 2. Build a binned-SAH BVH - this returns a permutation of face indices in BVH leaf order.
    /// 3. Materialize `SimpleTriangle` data cold arrays in BVH leaf order.
    /// 4. Precompute area CDF for importance sampling.
    pub fn new(positions: Vec<Point3>, face_indices: Vec<[u32; 3]>, material: Arc<Material>) -> Self {
        let n_faces = face_indices.len();
        let n_pos = positions.len();
        assert!(n_faces > 0, "Mesh must have at least one face");
        assert!(n_pos > 0, "positions must not be empty");

        // ----- 1. Per-face data keyed by ORIGINAL face index. -----
        let mut face_bboxes: Vec<AABB> = Vec::with_capacity(n_faces);
        let mut face_centroids: Vec<Vec3> = Vec::with_capacity(n_faces);

        for (fi, &[i0, i1, i2]) in face_indices.iter().enumerate() {
            assert!((i0 as usize) < n_pos && (i1 as usize) < n_pos && (i2 as usize) < n_pos, 
                "Face {} has out-of-bounds index: ({}, {}, {}) with positions length {}", fi, i0, i1, i2, n_pos);

            let p0: Vec3 = positions[i0 as usize];
            let p1 = positions[i1 as usize];
            let p2 = positions[i2 as usize];

            let tri_bbox: AABB = AABB::from_point_triplet(&p0, &p1, &p2);
            face_bboxes.push(tri_bbox);
            face_centroids.push((p0 + p1 + p2) / 3.0);

            debug_assert!(i0 != i1 && i1 != i2 && i2 != i0,
                "Face {} has duplicate vertex indices: ({}, {}, {})", fi, i0, i1, i2);
            debug_assert!(
                p0.is_finite() && p1.is_finite() && p2.is_finite(),
                "Non-finite vertex detected in face {}: p0={}, p1={}, p2={}", fi, p0, p1, p2
            );
        }

        // ----- 2. Build BVH, obtaining a permutation of face indices in BVH leaf ordering -----
        let (bvh_nodes, new_triangle_order) = build_mesh_bvh(&face_bboxes, &face_centroids);

        // ----- 3. Create hot-path triangle data and parallel "cold" arrays in BVH leaf order -----
        // BVH leaves point at at indices [start, start+count) inside `triangles` directly.
        let mut triangles: Vec<SimpleTriangle> = Vec::with_capacity(n_faces);
        let mut face_indices_reordered: Vec<[u32; 3]> = Vec::with_capacity(n_faces);
        let mut face_areas: Vec<f64> = Vec::with_capacity(n_faces);

        for &orig_face_index in &new_triangle_order {
            let [i0, i1, i2] = face_indices[orig_face_index as usize];
            let p0: Point3 = positions[i0 as usize];
            let p1: Point3 = positions[i1 as usize];
            let p2: Point3 = positions[i2 as usize];

            let e1: Vec3 = p1 - p0;
            let e2: Vec3 = p2 - p0;

            triangles.push(SimpleTriangle { p0, e1, e2 });
            face_indices_reordered.push([i0, i1, i2]);

            let area: f64 = 0.5 * Vec3::cross(&e1, &e2).length();
            face_areas.push(area);
        }

        // ----- 4. Normalize culumative area distribution for area-weighted sampling -----
        let total_area: f64 = face_areas.iter().sum();
        let inv_total_area: f64 = if total_area > 0.0 { 1.0 / total_area } else { 0.0 };
        let mut face_area_cdf: Vec<f64> = Vec::with_capacity(n_faces);
        let mut acc: f64 = 0.0;
        for &area in &face_areas {
            acc += area * inv_total_area;
            face_area_cdf.push(acc);
        }
        face_area_cdf[n_faces - 1] = 1.0; // Clamp last value to exactly 1.0

        // For default flat shading we don't need the original positions or per-vertex attributes,
        // everything intersection-related is already baked into `triangles`.
        // These slots are kept as `Option` so a future `with_smooth_shading` constructor
        // can populate them without layout changes.
        let geometry = MeshGeometry {
            triangles,
            bvh_nodes,
            face_areas,
            face_area_cdf,
            total_area,
            positions: None,
            face_indices: None,
            vertex_normals: None,
            vertex_uvs: None,
        };

        Self {
            geometry: Arc::new(geometry),
            material,
        }
    }

    /// Get the bounding box of the entire mesh.
    pub fn bounding_box(&self) -> &AABB { &self.geometry.bvh_nodes[0].bounding_box } // Root node's bbox covers the whole mesh
    /// Get the total number of triangles in the mesh.
    pub fn triangle_count(&self) -> usize { self.geometry.triangles.len() }
    /// Get the total surface area of the mesh.
    pub fn total_area(&self) -> f64 { self.geometry.total_area }
    
    /// Möller-Trumbore intersection of a single `SimpleTriangle`.
    /// Returns `Some((t, u, v))` on hit, where `(u, v)` are barycentric coordinates. Returns `None` on miss.
    #[inline]
    fn hit_triangle(triangle: &SimpleTriangle, r: &Ray, ray_t: &Interval) -> Option<(f64, f64, f64)> {
        let ray_cross_e2: Vec3 = Vec3::cross(&r.direction, &triangle.e2);
        let det: f64 = Vec3::dot(&triangle.e1, &ray_cross_e2);

        if det.abs() < EPSILON { return None; } // Ray is parallel to the plane

        let inv_det: f64 = 1.0 / det;
        let s: Vec3 = r.origin - triangle.p0;
        let u: f64 = inv_det * Vec3::dot(&s, &ray_cross_e2);
        if u < 0.0 || u > 1.0 { return None; } // Intersection outside triangle

        let s_cross_e1: Vec3 = Vec3::cross(&s, &triangle.e1);
        let v: f64 = inv_det * Vec3::dot(&r.direction, &s_cross_e1);
        if v < 0.0 || u + v > 1.0 { return None; } // Intersection outside triangle

        let t = inv_det * Vec3::dot(&triangle.e2, &s_cross_e1);
        if !ray_t.contains(t) { return None; } // Intersection outside ray bounds
        Some((t, u, v))
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let geometry: &MeshGeometry = &*self.geometry;
        
        // Iterative traversal stack. MAX_BVH_DEPTH is a safe upper bound.
        let mut stack: [u32; MAX_BVH_DEPTH] = [0; MAX_BVH_DEPTH];
        let mut stack_pointer: usize = 1; // Will be subtracted by 1 at the start of the loop, starts at 1 to process root node first.
        stack[0] = 0; // Start with root node index

        let mut closest_t: f64 = ray_t.max;
        let mut hit_anything: bool = false;
        let mut best_tri_index: usize = 0; // Index of the closest hit triangle in `geometry.triangles`.
        // Barycentric coordinates of the hit point on the triangle.
        let mut best_u: f64 = 0.0;
        let mut best_v: f64 = 0.0;

        while stack_pointer > 0 { // While the stack is not empty
            // Pop the next node from the stack
            stack_pointer -= 1;
            let node_index: usize = stack[stack_pointer] as usize;
            let node: &MeshBVHNode = &geometry.bvh_nodes[node_index];

            // Tighten the ray_t interval
            let mut t_interval: Interval = Interval::new(ray_t.min, closest_t);
            if !node.bounding_box.hit(r, &t_interval) { continue; } // Not intersection -> skip this node

            if node.is_leaf() {
                // Leaf node: linear scan over slice of `triangles`
                let start: usize = node.right_child_or_first_triangle as usize;
                let end: usize = start + node.triangle_count as usize;
                for i in start..end {
                    t_interval.max = closest_t; // Tighten bound
                    let triangle = &geometry.triangles[i];
                    if let Some((t, u, v)) = Self::hit_triangle(triangle, r, &t_interval) {
                        closest_t = t;
                        hit_anything = true;
                        best_tri_index = i;
                        best_u = u;
                        best_v = v;
                    }
                }
            } else {
                // Internal node: probe both children and visit the nearer one first
                // so we can cull the farther one once a hit shrinks closest_t.
                let left = node_index + 1;
                let right = node.right_child_or_first_triangle as usize;

                // Get the intersection t values for the child bounding boxes.
                let left_t = geometry.bvh_nodes[left].bounding_box.hit_with_t(r, &t_interval);
                let right_t = geometry.bvh_nodes[right].bounding_box.hit_with_t(r, &t_interval);

                match (left_t, right_t) {
                    (None, None) => {}
                    (Some(_), None) => { stack[stack_pointer] = left as u32; stack_pointer += 1; }
                    (None, Some(_)) => { stack[stack_pointer] = right as u32; stack_pointer += 1; }
                    (Some(lt), Some(rt)) => { // Push the far child first so the near child is popped next.
                        if lt <= rt {
                            stack[stack_pointer] = right as u32; stack_pointer += 1;
                            stack[stack_pointer] = left as u32; stack_pointer += 1;
                        } else {
                            stack[stack_pointer] = left as u32; stack_pointer += 1;
                            stack[stack_pointer] = right as u32; stack_pointer += 1;
                        }
                    }
                }
            }
        }

        if hit_anything {
            // Compute the face normal lazily: only for the closest hit triangle, and after confirming a hit
            // instead of caching a Vec3 on every triangle.
            let tri: &SimpleTriangle = &geometry.triangles[best_tri_index];
            let n: Vec3 = Vec3::unit_vector(&Vec3::cross(&tri.e1, &tri.e2));
            // Update hit record with hit information
            rec.t = closest_t;
            rec.point = r.at(closest_t);
            rec.material = Arc::clone(&self.material);
            rec.set_face_normal(r, &n);
            let _ = (best_u, best_v); // Silence unused variable warning for now
        }

        hit_anything
    }

    /// PDF value for a uniform-area sampler on the mesh surface: density is `1 / total_area`
    /// over the surface, so the solid-angle PDF is `dist^2 / (cos_theta * total_area)`.
    pub fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let geometry: &MeshGeometry = &*self.geometry;
        
        let ray = Ray::new(*origin, *direction);
        let mut rec = HitRecord::new();
        if !self.hit(&ray, &Interval::new(0.001, f64::INFINITY), &mut rec) {
            return 0.0;
        }
        
        let distance_squared: f64 = rec.t * rec.t * direction.length_squared();
        let cosine: f64 = (Vec3::dot(direction, &rec.normal) / direction.length()).abs();
        distance_squared / (cosine * geometry.total_area)
    }

    /// Sample a point on the mesh surface uniformly by area, returning the
    /// vector from `origin` to that point. Picks a face proportional to its area
    /// via binary search on the CDF, then a uniform point inside it 
    /// via the reflection trick.
    pub fn random(&self, origin: &Point3) -> Vec3 {
        let geometry: &MeshGeometry = &*self.geometry;

        // Pick a face proportional to its area
        let r: f64 = random_f64();
        let face_index: usize = geometry.face_area_cdf
            .partition_point(|&cum| cum < r);
        debug_assert!(face_index < geometry.triangles.len(), "Random sampling went out of bounds");
        let tri: &SimpleTriangle = &geometry.triangles[face_index];

        // Uniform sample inside the triangle in barycentric space.
        let mut u: f64 = random_f64();
        let mut v: f64 = random_f64();
        if u + v > 1.0 { // Reflect back into triangle if outside
            u = 1.0 - u;
            v = 1.0 - v;
        }
        let point = tri.p0 + u * tri.e1 + v * tri.e2;
        point - *origin
    }
}

impl From<TriangleMesh> for Hittable {
    fn from(mesh: TriangleMesh) -> Self {
        Hittable::TriangleMesh(mesh)
    }
}

// =====================================================================
// SAH BVH builder
// =====================================================================

/// Build the flat BVH. Returns `(bvh_nodes, triangle_order)` where `bvh_nodes` is a contiguous array
/// of `MeshBVHNode` in left-child-first order, and `triangle_order` is a permutation of face indices
/// that gives the BVH-leaf order of the source triangles. Leaves slice into this permutation
/// at build time.
fn build_mesh_bvh(face_bboxes: &[AABB], face_centroids: &[Point3]) -> (Vec<MeshBVHNode>, Vec<u32>) {
    let n: usize = face_bboxes.len();
    let mut new_triangle_order: Vec<u32> = (0..n as u32).collect();
    // Upper bound for the number of BVH nodes with leaf size L is 2 * ceil(n / L) - 1
    // Allocate a bit more to avoid reallocations during pushes.
    let mut nodes: Vec<MeshBVHNode> = Vec::with_capacity(2 * (n / MAX_TRIANGLES_PER_LEAF).max(1));

    if n > 0 {
        build_subtree(&mut nodes, &mut new_triangle_order, 0, face_bboxes, face_centroids, 0);
    }
    nodes.shrink_to_fit();
    (nodes, new_triangle_order)
}

/// Recursively build a subtree. `slice` is the portion of the permutation that
/// belongs to this subtree. `slice_offset` is its starting position in the global `triangle_order`,
/// used to fill leaf `right_child_or_first_triangle` so leaves can directly index `triangles` later.
fn build_subtree(
    nodes: &mut Vec<MeshBVHNode>,
    slice: &mut [u32],
    slice_offset: u32,
    face_bboxes: &[AABB],
    face_centroids: &[Point3],
    depth: usize,
) -> u32 {
    // Reserve this node's index now, we'll patch its fields after recursing.
    let node_index: u32 = nodes.len() as u32;
    nodes.push(MeshBVHNode {
        bounding_box: AABB::empty(),
        right_child_or_first_triangle: 0,
        triangle_count: 0,
    });

    // Compute this node's primitive AABB and the AABB of its centroids.
    // Centroid AABB drives the SAH binning
    // Primitive AABB drives the SAH costs + ray-AABB tests during BVH traversal.
    let mut bbox: AABB = AABB::empty();
    let mut centroid_bbox = AABB::empty();
    for &face_index in slice.iter() {
        bbox = AABB::merge(&bbox, &face_bboxes[face_index as usize]);
        let centroid: Vec3 = face_centroids[face_index as usize];
        centroid_bbox = AABB::merge_point(&centroid_bbox, &centroid);
    }

    let n = slice.len();

    // Helper to finalize this node as a leaf
    let make_leaf = |nodes: &mut Vec<MeshBVHNode>, bbox: AABB| {
        let node: &mut MeshBVHNode = &mut nodes[node_index as usize];
        node.bounding_box = bbox;
        node.right_child_or_first_triangle = slice_offset; // Point to the start of this leaf's triangles in the global order
        node.triangle_count = n as u32;
    };

    // Leaf node: small enough, or we hit the depth limit.
    if n <= MAX_TRIANGLES_PER_LEAF || depth + 1 >= MAX_BVH_DEPTH {
        make_leaf(nodes, bbox);
        return node_index;
    }

    // Search every (axis, bin boundary) pair for the lowest cost split.
    let split: Option<(usize, f64)> = find_best_split(slice, face_bboxes, face_centroids, &centroid_bbox, &bbox);
    let Some((split_axis, split_pos)) = split else {
        // No valid split found
        make_leaf(nodes, bbox);
        return node_index;
    };

    // Partition the slice in place by the chosen split position along `split_axis`.
    let mid: usize = partition_by_axis(slice, face_centroids, split_axis, split_pos);

    // If we get a degenerate split, panic for bug detection.
    if mid == 0 || mid == n { panic!("Degenerate split found during BVH SAH partitioning. Check `partition_by_axis` implementation."); }
        
    // Recurse: left is at node_index + 1 by construction (we push left first),
    // right is wherever it ends up and we patch it onto this node after the left subtree is built.
    let (left_slice, right_slice) = slice.split_at_mut(mid);
    let _left_index: u32 = build_subtree(nodes, left_slice, slice_offset, face_bboxes, face_centroids, depth + 1);
    let right_index: u32 = build_subtree(nodes, right_slice, slice_offset + mid as u32, face_bboxes, face_centroids, depth + 1);

    // Patch this internal node's fields now that the children are known.
    let node: &mut MeshBVHNode = &mut nodes[node_index as usize];
    node.bounding_box = bbox;
    node.right_child_or_first_triangle = right_index; // Right child index
    node.triangle_count = 0; // Internal node

    node_index
}

/// Struct representing a single bin for binned SAH.
#[derive(Clone, Copy)]
struct Bin {
    bbox: AABB,
    count: u32,
}

impl Bin {
    /// Create an empty bin with zero count and an empty bounding box.
    #[inline] fn empty() -> Self { Self { bbox: AABB::empty(), count: 0 } }
    /// Add a triangle's bounding box to this bin and increment the count.
    #[inline] fn add_bbox(&mut self, bbox: &AABB) {
        self.bbox = AABB::merge(&self.bbox, bbox);
        self.count += 1;
    }
    /// Merge two bins into one, combining their counts and bounding boxes.
    #[inline] fn merge(a: &Bin, b: &Bin) -> Bin {
        Bin { bbox: AABB::merge(&a.bbox, &b.bbox), count: a.count + b.count }
    }
}

/// Bin all faces by centroid along each axis, sweep left-to-right and right-to-left to get
/// the count and bounding box on each side of every possible split and evaluate the SAH cost.
/// Returns the best split found as `(axis, split_position)` or `None` if no split improves on 
/// the leaf cost baseline.
fn find_best_split(
    slice: &[u32],
    face_bboxes: &[AABB],
    face_centroids: &[Point3],
    centroid_bbox: &AABB,
    parent_bbox: &AABB,
) -> Option<(usize, f64)> {
    let n: usize = slice.len();
    let parent_sa: f64 = parent_bbox.surface_area();
    if parent_sa < EPSILON { return None; } // Degenerate case
    let leaf_cost: f64 = SAH_INTERSECT_COST * n as f64;

    let mut best_cost: f64 = leaf_cost;
    let mut best_split: Option<(usize, f64)> = None;

    for axis in 0..3 { // Permute over each axis
        let interval: &Interval = centroid_bbox.axis_interval(axis);
        let extent: f64 = interval.size();
        // Check for degenerate case where all centroids coincide on this axis.
        if extent < EPSILON { continue; }
        
        // ----- 1. Bin every face into one of SAH_BIN_COUNT bins by centroid coord. -----
        // bi = floor( (c - x.min) / (x.max - x.min) * B ) , clamped to [0, B-1]
        let mut bins = [Bin::empty(); SAH_BIN_COUNT];
        let scale = SAH_BIN_COUNT as f64 / extent; // Convert centroid coords to bin index

        for &face_index in slice {
            let centroid: f64 = face_centroids[face_index as usize][axis];
            let mut bin_index: isize = ((centroid - interval.min) * scale) as isize;
            bin_index = bin_index.clamp(0, (SAH_BIN_COUNT - 1) as isize);
            bins[bin_index as usize].add_bbox(&face_bboxes[face_index as usize]);
        }

        // ----- 2. Sweep over bins to find (bbox, count) on each side of every boundary. -----
        let mut left = [Bin::empty(); SAH_BIN_COUNT - 1]; // left[i] = merge of bins[0..=i]
        let mut right = [Bin::empty(); SAH_BIN_COUNT - 1]; // right[i] = merge of bins[i+1..B-1]

        let mut acc = Bin::empty();
        for i in 0..SAH_BIN_COUNT - 1 { // Left to right sweep
            acc = Bin::merge(&acc, &bins[i]);
            left[i] = acc;
        }
        let mut acc = Bin::empty();
        for i in (0..SAH_BIN_COUNT - 1).rev() { // Right to left sweep
            acc = Bin::merge(&acc, &bins[i + 1]);
            right[i] = acc;
        }

        // ----- 3. Evaluate SAH cost per split ------
        // C = C_trav + (SA_l / SA_p) * N_l * C_inter + (SA_r / SA_p) * N_r * C_inter
        for i in 0..SAH_BIN_COUNT - 1 { // Permute over each split boundary
            let l = &left[i];
            let r = &right[i];
            if l.count == 0 || r.count == 0 { continue; } // Invalid if one side is empty

            let l_n = l.count as f64; let r_n = r.count as f64;
            let l_sa = l.bbox.surface_area(); let r_sa = r.bbox.surface_area();
            let cost = SAH_TRAVERSE_COST + SAH_INTERSECT_COST * (l_sa * l_n + r_sa * r_n) / parent_sa;

            if cost < best_cost { // x_split = x_min + (bi+1)/B * (x_max - x_min)
                best_cost = cost;
                let split_pos = interval.min + (i as f64 + 1.0) / SAH_BIN_COUNT as f64 * extent; // Convert back to position
                best_split = Some((axis, split_pos));
            }
        }
    }

    best_split
}

/// Partition `slice` in place by the split position along the given axis, 
/// using the face centroids for comparison.
/// Returns the index of the first element in the right partition (i.e., the split point).
fn partition_by_axis(
    slice: &mut [u32],
    face_centroids: &[Point3],
    axis: usize,
    split_pos: f64,
) -> usize {
    let mut i = 0usize;
    let mut j = slice.len();
    while i < j {
        let centroid_i = face_centroids[slice[i] as usize][axis];
        if centroid_i < split_pos {
            i += 1;
        } else {
            j -= 1;
            slice.swap(i, j);
        }
    }
    i    
}