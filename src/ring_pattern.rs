use super::{
    color::Color, pattern::Pattern, point3d::Point3D, transform::Transform,
};

#[derive(Debug)]
pub struct RingPattern {
    a: Color,
    b: Color,
    /// Pattern -> Shape Transform
    transform: Transform,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        RingPattern {
            a,
            b,
            transform: Transform::identity(),
        }
    }
}

impl Pattern for RingPattern {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn pattern_at(&self, p: &Point3D) -> Color {
        let distance = (&p.x * &p.x + &p.z * &p.z).floor() as i32;
        if distance % 2 == 0 {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = RingPattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 1.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(0.708, 0.0, 0.708))
        );
    }
}
