pub mod camera;
pub mod canvas;
pub mod checkers_pattern;
pub mod color;
pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod gradient_pattern;
pub mod intersection;
pub mod intersection_state;
pub mod light;
pub mod material;
pub mod matrix4x4;
pub mod node;
pub mod pattern;
pub mod plane;
pub mod point3d;
pub mod ray;
pub mod ring_pattern;
pub mod shape;
pub mod sphere;
pub mod stripe_pattern;
pub mod transform;
pub mod vector3d;
pub mod world;

pub type FLOAT = f64;
const EPSILON: FLOAT = 0.00001;
const INFINITY: FLOAT = std::f64::INFINITY;

fn approx_eq(a: FLOAT, b: FLOAT) -> bool {
    (a - b).abs() < EPSILON
}
