pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersection;
pub mod intersection_state;
pub mod light;
pub mod material;
pub mod matrix4x4;
pub mod point3d;
pub mod ray;
pub mod sphere;
pub mod transform;
pub mod vector3d;
pub mod world;

pub type FLOAT = f64;
const EPSILON: FLOAT = 0.00001;

fn approx_eq(a: FLOAT, b: FLOAT) -> bool {
    (a - b).abs() < EPSILON
}
