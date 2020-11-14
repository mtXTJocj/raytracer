use super::{color::Color, light::Light, point3d::Point3D, vector3d::Vector3D};

/// マテリアル
#[derive(Debug)]
pub struct Material {
    /// 色
    pub color: Color,
    /// 環境光の強さ
    pub ambient: f32,
    /// 拡散反射光の強さ
    pub diffuse: f32,
    /// 鏡面反射光の強さ
    pub specular: f32,
    /// 鏡面反射光の広がり。大きい程、狭く強い。
    pub shininess: f32,
}

impl Material {
    /// Material を作成する
    pub fn new() -> Self {
        Material {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    /// ライティングの計算を行う。
    ///
    /// # Argumets
    ///
    /// * `light` - 光源
    /// * `point` - 計算を行うオブジェクト上の点
    /// * `eyev` - 視線ベクトル
    /// * `normalv` - point における法線ベクトル
    pub fn lighting(
        &self,
        light: &Light,
        point: &Point3D,
        eyev: &Vector3D,
        normalv: &Vector3D,
    ) -> Color {
        let effective_color = &self.color * light.intensity();
        let mut lightv = light.position() - point;
        lightv.normalize();
        let ambient = &effective_color * self.ambient;

        let diffuse;
        let specular;
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::BLACK;
            specular = Color::BLACK;
        } else {
            diffuse = &(&effective_color * self.diffuse) * light_dot_normal;
            let reflectv = (-&lightv).reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                specular = Color::BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = self.specular * factor * light.intensity();
            }
        }

        &(&ambient + &diffuse) + &specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(Color::WHITE, m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200.0, m.shininess);
    }

    #[test]
    fn lihgting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, -10.0), Color::WHITE);

        let result = m.lighting(&light, &p, &eyev, &normalv);
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45deg() {
        let m = Material::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, -10.0), Color::WHITE);

        let result = m.lighting(&light, &p, &eyev, &normalv);
        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45deg() {
        let m = Material::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 10.0, -10.0), Color::WHITE);

        let result = m.lighting(&light, &p, &eyev, &normalv);
        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, -2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 10.0, -10.0), Color::WHITE);

        let result = m.lighting(&light, &p, &eyev, &normalv);
        assert_eq!(Color::new(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, 10.0), Color::WHITE);

        let result = m.lighting(&light, &p, &eyev, &normalv);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
}
