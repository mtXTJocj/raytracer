use super::{color::Color, point3d::Point3D};

/// 点光源
#[derive(Debug)]
pub struct Light {
    /// 光源位置
    position: Point3D,
    /// 色
    intensity: Color,
}

impl Light {
    /// 点光源を作成する
    ///
    /// # Argumets
    ///
    /// * `position` - 光源位置
    /// * `intensity` - 色
    pub fn new(position: Point3D, intensity: Color) -> Self {
        Light {
            position,
            intensity,
        }
    }

    /// 光源位置を取得する
    pub fn position(&self) -> &Point3D {
        &self.position
    }

    /// 色を取得する
    pub fn intensity(&self) -> &Color {
        &self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_posiiton_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point3D::new(0.0, 0.0, 0.0);

        let light = Light::new(position.clone(), intensity.clone());

        assert_eq!(position, *light.position());
        assert_eq!(intensity, *light.intensity());
    }
}
