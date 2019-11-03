pub mod canvas;
pub mod color;
pub mod point3d;
pub mod vector3d;

const EPSILON: f32 = 0.0001;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}
