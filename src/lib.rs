pub mod canvas;
pub mod color;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix4x4;
pub mod point3d;
pub mod ray;
pub mod sphere;
pub mod transform;
pub mod vector3d;

const EPSILON: f32 = 0.0001;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}
