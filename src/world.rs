use super::{
    color::Color, intersection::Intersection,
    intersection_state::IntersectionState, light::Light, ray::Ray,
    sphere::Sphere,
};

/// レンダリングに用いるライトとオブジェクトを集約する
#[derive(Debug)]
pub struct World {
    /// ライト
    lights: Vec<Light>,
    /// オブジェクト
    shapes: Vec<Sphere>,
}

impl World {
    /// 新規に World を作成する
    pub fn new() -> Self {
        World {
            lights: vec![],
            shapes: vec![],
        }
    }

    /// ライトを追加する
    ///
    /// # Arguments
    ///
    /// * `light` - 追加するライト
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// オブジェクトを追加する
    ///
    /// # Arguments
    ///
    /// * `sphere` - 追加するオブジェクト
    pub fn add_shape(&mut self, sphere: Sphere) {
        self.shapes.push(sphere);
    }

    /// Ray とオブジェクトの交差判定を行い、交差情報のリストを返す。
    /// 返された交差情報は Ray の起点を基準にソートされている。
    ///
    /// # Arguments
    ///
    /// * `ray` - 判定対象となる Ray
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];
        for shape in &self.shapes {
            let mut xs = shape.intersect(ray);
            intersections.append(&mut xs);
        }

        intersections.sort_unstable_by(|i1, i2| {
            if i1.t < i2.t {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        intersections
    }

    /// Ray がヒットした点における色を返す。
    ///
    /// # Arguments
    ///
    /// * `intersection_state` - 計算に必要な交点情報
    fn shade_hit(&self, intersection_state: &IntersectionState) -> Color {
        let mut c = Color::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            c = &c
                + &intersection_state.object.material().lighting(
                    light,
                    &intersection_state.point,
                    &intersection_state.eyev,
                    &intersection_state.normalv,
                )
        }
        c
    }

    /// Ray に対応する色を返す。ヒットしなかった場合、黒を返す
    ///
    /// # Arguments
    ///
    /// * `r` - Ray
    pub fn color_at(&self, r: &Ray) -> Color {
        let xs = self.intersect(r);
        if xs.len() > 0 {
            let is = IntersectionState::new(&xs[0], r);
            self.shade_hit(&is)
        } else {
            Color::BLACK
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            approx_eq, camera::Camera, color::Color, material::Material,
            point3d::Point3D, transform::Transform, vector3d::Vector3D,
        },
        *,
    };

    fn default_world() -> World {
        let mut w = World::new();

        let light = Light::new(
            Point3D::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        w.add_light(light);

        let mut sphere = Sphere::new();
        let mut material = Material::new();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        *sphere.material_mut() = material;
        w.add_shape(sphere);

        let mut sphere = Sphere::new();
        *sphere.transform_mut() = Transform::scaling(0.5, 0.5, 0.5);
        w.add_shape(sphere);
        return w;
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = w.intersect(&r);
        assert_eq!(4, xs.len());
        assert!(approx_eq(4.0, xs[0].t));
        assert!(approx_eq(4.5, xs[1].t));
        assert!(approx_eq(5.5, xs[2].t));
        assert!(approx_eq(6.0, xs[3].t));
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let shape = &w.shapes[0];
        let i = Intersection {
            t: 4.0,
            object: shape,
        };
        let comps = IntersectionState::new(&i, &r);

        let c = w.shade_hit(&comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights[0] = Light::new(Point3D::new(0.0, 0.25, 0.0), Color::WHITE);
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let shape = &w.shapes[1];
        let i = Intersection {
            t: 0.5,
            object: shape,
        };
        let comps = IntersectionState::new(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn shading_an_intersection_with_two_lights() {
        let mut w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let l = Light::new(
            Point3D::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        w.add_light(l);

        let shape = &w.shapes[0];
        let i = Intersection {
            t: 4.0,
            object: shape,
        };
        let comps = IntersectionState::new(&i, &r);

        let c = w.shade_hit(&comps);
        assert_eq!(Color::new(0.76132, 0.95166, 0.5710), c);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );
        let c = w.color_at(&r);
        assert_eq!(Color::BLACK, c);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let c = w.color_at(&r);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn the_color_with_an_intersection_behinde_a_ray() {
        let mut w = default_world();
        w.shapes[0].material_mut().ambient = 1.0;
        w.shapes[1].material_mut().ambient = 1.0;
        let r = Ray::new(
            Point3D::new(0.0, 0.0, 0.75),
            Vector3D::new(0.0, 0.0, -1.0),
        );
        let c = w.color_at(&r);
        assert_eq!(w.shapes[0].material().color, c);
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let mut c = Camera::new(11, 11, std::f32::consts::FRAC_PI_2);
        let from = Point3D::new(0.0, 0.0, -5.0);
        let to = Point3D::new(0.0, 0.0, 0.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);
        *c.transform_mut() = Transform::view_transform(&from, &to, &up);
        let image = c.render(&w);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), *image.color_at(5, 5));
    }
}
