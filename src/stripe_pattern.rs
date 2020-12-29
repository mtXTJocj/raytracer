use super::{
    color::Color, pattern::Pattern, point3d::Point3D, transform::Transform,
};

/// x 軸方向に変化する縞模様のパターン
#[derive(Debug)]
pub struct StripePattern {
    a: Color,
    b: Color,
    /// Pattern -> Shape Transform
    transform: Transform,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern {
            a,
            b,
            transform: Transform::identity(),
        }
    }
}

impl Pattern for StripePattern {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn pattern_at(&self, p: &Point3D) -> Color {
        let x = p.x.floor() as i32;
        if x % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{shape::Shape, sphere::Sphere, transform::Transform},
        *,
    };

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(Color::WHITE, pattern.a);
        assert_eq!(Color::BLACK, pattern.b);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 2.0, 0.0))
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 2.0))
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.9, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(-0.1, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(-1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(-1.1, 0.0, 0.0))
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Sphere::new();
        *object.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        let c = pattern.pattern_at_shape(&object, &Point3D::new(1.5, 0.0, 0.0));

        assert_eq!(Color::WHITE, c);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::new();
        let mut pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        *pattern.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);
        let c = pattern.pattern_at_shape(&object, &Point3D::new(1.5, 0.0, 0.0));

        assert_eq!(Color::WHITE, c);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = Sphere::new();
        *object.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);
        let mut pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        *pattern.transform_mut() = Transform::translation(0.5, 0.0, 0.0);
        let c = pattern.pattern_at_shape(&object, &Point3D::new(2.5, 0.0, 0.0));

        assert_eq!(Color::WHITE, c);
    }
}
