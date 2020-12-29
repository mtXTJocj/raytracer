use super::{
    color::Color, light::Light, pattern::Pattern, point3d::Point3D,
    shape::Shape, vector3d::Vector3D, FLOAT,
};

/// マテリアル
#[derive(Debug)]
pub struct Material {
    /// 色
    pub color: Color,
    /// 環境光の強さ
    pub ambient: FLOAT,
    /// 拡散反射光の強さ
    pub diffuse: FLOAT,
    /// 鏡面反射光の強さ
    pub specular: FLOAT,
    /// 鏡面反射光の広がり。大きい程、狭く強い。
    pub shininess: FLOAT,
    /// パターン。None の場合は使用しない。
    pattern: Option<Box<dyn Pattern>>,
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
            pattern: None,
        }
    }

    /// パターンを取得する
    pub fn pattern(&self) -> &Option<Box<dyn Pattern>> {
        &self.pattern
    }

    /// パターンを取得する
    pub fn pattern_mut(&mut self) -> &mut Option<Box<dyn Pattern>> {
        &mut self.pattern
    }

    /// ライティングの計算を行う。
    ///
    /// # Argumets
    ///
    /// * `light` - 光源
    /// * `point` - 計算を行うオブジェクト上の点
    /// * `eyev` - 視線ベクトル
    /// * `normalv` - point における法線ベクトル
    /// * `in_shadow` - 影の中にいるか
    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: &Light,
        point: &Point3D,
        eyev: &Vector3D,
        normalv: &Vector3D,
        in_shadow: bool,
    ) -> Color {
        let color = match self.pattern {
            Some(ref pattern) => pattern.pattern_at_shape(object, &point),
            None => self.color,
        };

        let effective_color = &color * light.intensity();
        let mut lightv = light.position() - point;
        lightv.normalize();
        let ambient = &effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }

        let diffuse;
        let specular;
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            return ambient;
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
    use super::{
        super::{sphere::Sphere, stripe_pattern::StripePattern},
        *,
    };

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
        let object = Sphere::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, -10.0), Color::WHITE);

        let result = m.lighting(&object, &light, &p, &eyev, &normalv, false);
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45deg() {
        let m = Material::new();
        let object = Sphere::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(
            0.0,
            2f32.sqrt() as FLOAT / 2.0,
            -2f32.sqrt() as FLOAT / 2.0,
        );
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, -10.0), Color::WHITE);

        let result = m.lighting(&object, &light, &p, &eyev, &normalv, false);
        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45deg() {
        let m = Material::new();
        let object = Sphere::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 10.0, -10.0), Color::WHITE);

        let result = m.lighting(&object, &light, &p, &eyev, &normalv, false);
        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let object = Sphere::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(
            0.0,
            -2f32.sqrt() as FLOAT / 2.0,
            -2f32.sqrt() as FLOAT / 2.0,
        );
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 10.0, -10.0), Color::WHITE);

        let result = m.lighting(&object, &light, &p, &eyev, &normalv, false);
        assert_eq!(Color::new(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let object = Sphere::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, 10.0), Color::WHITE);

        let result = m.lighting(&object, &light, &p, &eyev, &normalv, false);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let object = Sphere::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, -10.0), Color::WHITE);
        let in_shadow = true;

        let result =
            m.lighting(&object, &light, &p, &eyev, &normalv, in_shadow);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = Material::new();
        let object = Sphere::new();
        *m.pattern_mut() =
            Some(Box::new(StripePattern::new(Color::WHITE, Color::BLACK)));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Vector3D::new(0.0, 0.0, -1.0);
        let normalv = Vector3D::new(0.0, 0.0, -1.0);
        let light = Light::new(Point3D::new(0.0, 0.0, -10.0), Color::WHITE);

        let c1 = m.lighting(
            &object,
            &light,
            &Point3D::new(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        let c2 = m.lighting(
            &object,
            &light,
            &Point3D::new(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );

        assert_eq!(Color::WHITE, c1);
        assert_eq!(Color::BLACK, c2);
    }
}
