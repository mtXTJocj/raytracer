use super::{color::Color, point3d::Point3D};

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
