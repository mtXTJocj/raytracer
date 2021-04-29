use super::{
    color::Color, pattern::Pattern, point3d::Point3D, transform::Transform,
};

/// x 軸方向に変化するグラデーションパターン
#[derive(Debug)]
pub struct GradientPattern {
    a: Color,
    b: Color,
    /// Pattern -> Shape Transform
    transform: Transform,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        GradientPattern {
            a,
            b,
            transform: Transform::identity(),
        }
    }
}

impl Pattern for GradientPattern {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn pattern_at(&self, p: &Point3D) -> Color {
        let distance = &self.b - &self.a;
        let fraction = p.x - p.x.floor();

        &self.a + &(&distance * fraction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(Color::WHITE, Color::BLACK);

        assert_eq!(
            Color::WHITE,
            pattern.pattern_at(&Point3D::new(0.0, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            pattern.pattern_at(&Point3D::new(0.25, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            pattern.pattern_at(&Point3D::new(0.5, 0.0, 0.0))
        );
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            pattern.pattern_at(&Point3D::new(0.75, 0.0, 0.0))
        );
    }
}
