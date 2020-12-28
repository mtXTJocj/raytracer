use super::{
    color::Color, pattern::Pattern, point3d::Point3D, transform::Transform,
};

#[derive(Debug)]
pub struct CheckersPattern {
    a: Color,
    b: Color,
    /// Pattern -> Shape Transform
    transform: Transform,
}

impl CheckersPattern {
    pub fn new(a: Color, b: Color) -> Self {
        CheckersPattern {
            a,
            b,
            transform: Transform::identity(),
        }
    }
}

impl Pattern for CheckersPattern {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn pattern_at(&self, p: &Point3D) -> Color {
        if (p.x.floor() + p.y.floor() + p.z.floor()) as i32 % 2 == 0 {
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
    fn checkers_should_repeat_in_x() {
        let pattern = CheckersPattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.99, 0.0, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(1.01, 0.0, 0.0))
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = CheckersPattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.99, 0.0))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(0.0, 1.01, 0.0))
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = CheckersPattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.99))
        );
        assert_eq!(
            Color::BLACK,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 1.01))
        );
    }
}
