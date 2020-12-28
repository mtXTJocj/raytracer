use super::{
    color::Color, point3d::Point3D, shape::Shape, transform::Transform,
};
use std::fmt::Debug;

pub trait Pattern: Debug {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;

    fn pattern_at(&self, p: &Point3D) -> Color;
    fn pattern_at_shape(&self, shape: &dyn Shape, p: &Point3D) -> Color {
        let local_p = shape.transform().inv() * p;
        let pattern_p = self.transform().inv() * &local_p;
        self.pattern_at(&pattern_p)
    }
}
