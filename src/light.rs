use super::{color::Color, point3d::Point3D};

#[derive(Debug)]
pub struct Light {
    position: Point3D,
    intensity: Color,
}

impl Light {
    pub fn new(position: Point3D, intensity: Color) -> Self {
        Light {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> &Point3D {
        &self.position
    }

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
