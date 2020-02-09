use super::{color::Color, point3d::Point3D};

#[derive(Debug)]
pub struct Light {
    intensity: Color,
    position: Point3D,
}

impl Light {
    pub fn new(intensity: Color, position: Point3D) -> Self {
        Light {
            intensity,
            position,
        }
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
    }

    pub fn position(&self) -> &Point3D {
        &self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_posiiton_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point3D::new(0.0, 0.0, 0.0);

        let light = Light::new(intensity.clone(), position.clone());

        assert_eq!(intensity, *light.intensity());
        assert_eq!(position, *light.position());
    }
}
