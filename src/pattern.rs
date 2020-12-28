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

#[cfg(test)]
mod tests {
    use super::{super::sphere::Sphere, *};

    #[derive(Debug)]
    struct TestPattern {
        transform: Transform,
    }

    impl TestPattern {
        fn new() -> Self {
            TestPattern {
                transform: Transform::identity(),
            }
        }
    }

    impl Pattern for TestPattern {
        fn transform(&self) -> &Transform {
            &self.transform
        }

        fn transform_mut(&mut self) -> &mut Transform {
            &mut self.transform
        }

        fn pattern_at(&self, p: &Point3D) -> Color {
            Color::new(p.x, p.y, p.z)
        }
    }

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = TestPattern::new();

        assert_eq!(&Transform::identity(), pattern.transform());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut pattern = TestPattern::new();
        *pattern.transform_mut() = Transform::translation(1.0, 2.0, 3.0);

        assert_eq!(&Transform::translation(1.0, 2.0, 3.0), pattern.transform());
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let mut shape = Sphere::new();
        *shape.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);
        let pattern = TestPattern::new();
        let c = pattern.pattern_at_shape(&shape, &Point3D::new(2.0, 3.0, 4.0));

        assert_eq!(Color::new(1.0, 1.5, 2.0), c);
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut shape = Sphere::new();
        *shape.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);
        let mut pattern = TestPattern::new();
        *pattern.transform_mut() = Transform::translation(0.5, 1.0, 1.5);
        let c = pattern.pattern_at_shape(&shape, &Point3D::new(2.5, 3.0, 3.5));

        assert_eq!(Color::new(0.75, 0.5, 0.25), c);
    }
}
