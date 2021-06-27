use super::{color::Color, node::Node, point3d::Point3D, transform::Transform};
use std::fmt::Debug;

pub trait Pattern: Debug {
    /// self に対する変換を取得する
    fn transform(&self) -> &Transform;
    /// self に対する変換を取得する
    fn transform_mut(&mut self) -> &mut Transform;

    /// 点 p におけるパターンの色を返す。
    ///
    /// # Argumets
    /// * `p` - pattern 座標系における点
    fn pattern_at(&self, p: &Point3D) -> Color;
    /// World 上の点 p におけるパターンの色を返す。
    ///
    /// # Argumets
    /// * `p` - World 座標系における点
    fn pattern_at_shape(&self, node: &Node, p: &Point3D) -> Color {
        let local_p = node.transform().inv() * p;
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
        let mut node = Node::new(Box::new(Sphere::new()));
        node.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern::new();
        let c = pattern.pattern_at_shape(&node, &Point3D::new(2.0, 3.0, 4.0));

        assert_eq!(Color::new(1.0, 1.5, 2.0), c);
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut node = Node::new(Box::new(Sphere::new()));
        node.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        let mut pattern = TestPattern::new();
        *pattern.transform_mut() = Transform::translation(0.5, 1.0, 1.5);
        let c = pattern.pattern_at_shape(&node, &Point3D::new(2.5, 3.0, 3.5));

        assert_eq!(Color::new(0.75, 0.5, 0.25), c);
    }
}
