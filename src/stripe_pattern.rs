use super::{color::Color, point3d::Point3D, shape::Shape};

#[derive(Debug)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripePattern { a, b }
    }

    pub fn stripe_at(&self, p: &Point3D) -> Color {
        let x = p.x.floor() as i32;
        if x % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, shape: &dyn Shape, p: &Point3D) -> Color {
        let local_p = shape.transform().inv() * p;
        self.stripe_at(&local_p)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{sphere::Sphere, transform::Transform},
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
            pattern.stripe_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(0.0, 2.0, 0.0))
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(0.0, 0.0, 2.0))
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(0.9, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.stripe_at(&Point3D::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.stripe_at(&Point3D::new(-0.1, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.stripe_at(&Point3D::new(-1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.stripe_at(&Point3D::new(-1.1, 0.0, 0.0))
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Sphere::new();
        *object.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);
        let pattern = StripePattern::new(Color::WHITE, Color::BLACK);
        let c = pattern.stripe_at_object(&object, &Point3D::new(1.5, 0.0, 0.0));

        assert_eq!(Color::WHITE, c);
    }
}
