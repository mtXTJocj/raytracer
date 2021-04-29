use super::{
    color::Color,
    intersection::{hit, Intersection},
    intersection_state::IntersectionState,
    light::Light,
    point3d::Point3D,
    ray::Ray,
    shape::Shape,
};
use std::boxed::Box;

/// レンダリングに用いるライトとオブジェクトを集約する
#[derive(Debug)]
pub struct World {
    /// ライト
    lights: Vec<Light>,
    /// オブジェクト
    shapes: Vec<Box<dyn Shape>>,
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
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
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
    /// * `remaining` - 再帰の最大深さまでの残り回数
    fn shade_hit(
        &self,
        intersection_state: &IntersectionState,
        remaining: usize,
    ) -> Color {
        let mut surface = Color::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            let is_shadowed =
                self.is_shadowed(&intersection_state.over_point, light);
            surface = &surface
                + &intersection_state.object.material().lighting(
                    intersection_state.object,
                    light,
                    &intersection_state.over_point,
                    &intersection_state.eyev,
                    &intersection_state.normalv,
                    is_shadowed,
                );
        }
        let reflected = self.reflected_color(&intersection_state, remaining);
        let refracted = self.refracted_color(&intersection_state, remaining);

        if intersection_state.object.material().reflective > 0.0
            && intersection_state.object.material().transparency > 0.0
        {
            let reflectance = intersection_state.schlick();
            &(&surface + &(&reflected * reflectance))
                + &(&refracted * (1.0 - reflectance))
        } else {
            &(&surface + &reflected) + &refracted
        }
    }

    /// Ray に対応する色を返す。ヒットしなかった場合、黒を返す
    ///
    /// # Arguments
    ///
    /// * `r` - Ray
    /// * `remaining` - 再帰の最大深さまでの残り回数
    pub fn color_at(&self, r: &Ray, remaining: usize) -> Color {
        let xs = self.intersect(r);
        if let Some(ref nearest) = hit(&xs) {
            let is = IntersectionState::new(nearest, r, &xs);
            self.shade_hit(&is, remaining)
        } else {
            Color::BLACK
        }
    }

    /// p と light の間に遮蔽物があるか
    ///
    /// # Arguments
    ///
    /// * `p` - 位置
    /// * `light` - ライト
    fn is_shadowed(&self, p: &Point3D, light: &Light) -> bool {
        let mut direction = light.position() - p;
        let distance = direction.magnitude();
        direction.normalize();

        let r = Ray::new(p.clone(), direction);
        let intersections = self.intersect(&r);
        if let Some(nearest) = hit(&intersections) {
            if nearest.t < distance {
                return true;
            }
        }
        false
    }

    /// 反射成分の色を計算する。
    ///
    /// # Arguments
    ///
    /// * `is` - 反射する点の情報
    /// * `remaining` - 再帰の最大深さまでの残り回数
    fn reflected_color(
        &self,
        is: &IntersectionState,
        remaining: usize,
    ) -> Color {
        if is.object.material().reflective == 0.0 {
            // 光を全く反射しない場合
            return Color::BLACK;
        }
        if remaining <= 0 {
            return Color::BLACK;
        }

        let reflect_ray = Ray::new(is.over_point.clone(), is.reflectv.clone());
        let color = self.color_at(&reflect_ray, remaining - 1);

        &color * is.object.material().reflective
    }

    /// 屈折成分の色を計算する。
    ///
    /// # Arguments
    ///
    /// * `is` - 屈折する点の情報
    /// * `remaining` - 再帰の最大深さまでの残り回数
    fn refracted_color(
        &self,
        is: &IntersectionState,
        remaining: usize,
    ) -> Color {
        if is.object.material().transparency == 0.0 {
            // 不透明な場合
            return Color::BLACK;
        }
        if remaining <= 0 {
            return Color::BLACK;
        }

        let n_ratio = is.n1 / is.n2;
        let cos_i = is.eyev.dot(&is.normalv);
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            // total internal reflection
            return Color::BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction =
            &(&is.normalv * (n_ratio * cos_i - cos_t)) - &(&is.eyev * n_ratio);
        let r = Ray::new(is.under_point.clone(), direction);
        &self.color_at(&r, remaining - 1) * is.object.material().transparency
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            approx_eq, camera::Camera, color::Color, material::Material,
            pattern::Pattern, plane::Plane, sphere::Sphere,
            transform::Transform, vector3d::Vector3D, FLOAT,
        },
        *,
    };

    #[derive(Debug)]
    struct TestPattern {
        transform: Transform,
    }

    impl TestPattern {
        fn new() -> Self {
            TestPattern {
                transform: Transform::identity(),
            }
        }
    }

    impl Pattern for TestPattern {
        fn transform(&self) -> &Transform {
            &self.transform
        }

        fn transform_mut(&mut self) -> &mut Transform {
            &mut self.transform
        }

        fn pattern_at(&self, p: &Point3D) -> Color {
            Color::new(p.x, p.y, p.z)
        }
    }

    fn default_world() -> World {
        let mut w = World::new();

        let light = Light::new(
            Point3D::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );
        w.add_light(light);

        let mut sphere = Box::new(Sphere::new());
        let mut material = Material::new();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        *sphere.material_mut() = material;
        w.add_shape(sphere);

        let mut sphere = Box::new(Sphere::new());
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
        let shape = w.shapes[0].as_ref();
        let i = Intersection {
            t: 4.0,
            object: shape,
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);

        let c = w.shade_hit(&comps, 1);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights[0] = Light::new(Point3D::new(0.0, 0.25, 0.0), Color::WHITE);
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let shape = w.shapes[1].as_ref();
        let i = Intersection {
            t: 0.5,
            object: shape,
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);
        let c = w.shade_hit(&comps, 1);
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

        let shape = w.shapes[0].as_ref();
        let i = Intersection {
            t: 4.0,
            object: shape,
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);

        let c = w.shade_hit(&comps, 1);
        assert_eq!(Color::new(0.76132, 0.95166, 0.5710), c);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );
        let c = w.color_at(&r, 1);
        assert_eq!(Color::BLACK, c);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let c = w.color_at(&r, 1);
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
        let c = w.color_at(&r, 1);
        assert_eq!(w.shapes[1].material().color, c);
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let mut c = Camera::new(11, 11, std::f32::consts::FRAC_PI_2 as FLOAT);
        let from = Point3D::new(0.0, 0.0, -5.0);
        let to = Point3D::new(0.0, 0.0, 0.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);
        *c.transform_mut() = Transform::view_transform(&from, &to, &up);
        let image = c.render(&w);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), *image.color_at(5, 5));
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = Point3D::new(0.0, 10.0, 0.0);

        assert_eq!(false, w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = Point3D::new(10.0, -10.0, 10.0);

        assert_eq!(true, w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = Point3D::new(-20.0, 20.0, -20.0);

        assert_eq!(false, w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = Point3D::new(-2.0, 2.0, -2.0);

        assert_eq!(false, w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        let light = Light::new(Point3D::new(0.0, 0.0, -10.0), Color::WHITE);
        w.add_light(light);
        let s1 = Box::new(Sphere::new());
        w.add_shape(s1);
        let mut s2 = Box::new(Sphere::new());
        *s2.transform_mut() = Transform::translation(0.0, 0.0, 10.0);
        w.add_shape(s2);

        let r =
            Ray::new(Point3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, 1.0));
        let i = Intersection {
            t: 4.0,
            object: w.shapes[1].as_ref(),
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);
        let c = w.shade_hit(&comps, 1);

        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn the_reflected_color_for_a_non_reflective_material() {
        let mut w = default_world();
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        w.shapes[1].material_mut().ambient = 1.0;
        let i = Intersection {
            t: 1.0,
            object: w.shapes[1].as_ref(),
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);
        let color = w.reflected_color(&comps, 1);

        assert_eq!(Color::BLACK, color);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let mut shape = Plane::new();
        shape.material_mut().reflective = 0.5;
        *shape.transform_mut() = Transform::translation(0.0, -1.0, 0.0);
        w.add_shape(Box::new(shape));
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -3.0),
            Vector3D::new(
                0.0,
                -2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
            ),
        );
        let i = Intersection {
            t: 2f32.sqrt() as FLOAT,
            object: w.shapes[2].as_ref(),
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);
        let color = w.reflected_color(&comps, 1);

        assert_eq!(Color::new(0.19033, 0.23791, 0.14274), color);
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = default_world();
        let mut shape = Plane::new();
        shape.material_mut().reflective = 0.5;
        *shape.transform_mut() = Transform::translation(0.0, -1.0, 0.0);
        w.add_shape(Box::new(shape));
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -3.0),
            Vector3D::new(
                0.0,
                -2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
            ),
        );
        let i = Intersection {
            t: 2f32.sqrt() as FLOAT,
            object: w.shapes[2].as_ref(),
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);
        let color = w.shade_hit(&comps, 1);

        assert_eq!(Color::new(0.87676, 0.92434, 0.82918), color);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.add_light(Light::new(Point3D::new(0.0, 0.0, 0.0), Color::WHITE));

        let mut lower = Plane::new();
        lower.material_mut().reflective = 1.0;
        *lower.transform_mut() = Transform::translation(0.0, -1.0, 0.0);
        w.add_shape(Box::new(lower));

        let mut upper = Plane::new();
        upper.material_mut().reflective = 1.0;
        *upper.transform_mut() = Transform::translation(0.0, 1.0, 0.0);
        w.add_shape(Box::new(upper));

        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));

        let _ = w.color_at(&r, 1);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        let mut shape = Plane::new();
        shape.material_mut().reflective = 0.5;
        *shape.transform_mut() = Transform::translation(0.0, -1.0, 0.0);

        w.add_shape(Box::new(shape));
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -3.0),
            Vector3D::new(
                0.0,
                -2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
            ),
        );
        let i = Intersection {
            t: 2f32.sqrt() as FLOAT,
            object: w.shapes[2].as_ref(),
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);
        let color = w.reflected_color(&comps, 0);

        assert_eq!(Color::BLACK, color);
    }

    #[test]
    fn the_refracted_color_with_an_opeque_surface() {
        let w = default_world();
        let shape = &w.shapes[0];
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let xs = vec![
            Intersection {
                t: 4.0,
                object: shape.as_ref(),
            },
            Intersection {
                t: 6.0,
                object: shape.as_ref(),
            },
        ];
        let comps = IntersectionState::new(&xs[0], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(Color::BLACK, c);
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut w = default_world();
        w.shapes[0].material_mut().transparency = 1.0;
        w.shapes[0].material_mut().refractive_index = 1.5;
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let xs = vec![
            Intersection {
                t: 4.0,
                object: w.shapes[0].as_ref(),
            },
            Intersection {
                t: 6.0,
                object: w.shapes[0].as_ref(),
            },
        ];
        let comps = IntersectionState::new(&xs[0], &r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(Color::BLACK, c);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = default_world();
        w.shapes[0].material_mut().transparency = 1.0;
        w.shapes[0].material_mut().refractive_index = 1.5;
        let r = Ray::new(
            Point3D::new(0.0, 0.0, 2f32.sqrt() as FLOAT / 2.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );
        let xs = vec![
            Intersection {
                t: -2f32.sqrt() as FLOAT / 2.0,
                object: w.shapes[0].as_ref(),
            },
            Intersection {
                t: 2f32.sqrt() as FLOAT / 2.0,
                object: w.shapes[0].as_ref(),
            },
        ];

        let comps = IntersectionState::new(&xs[1], &r, &xs);
        let c = w.refracted_color(&comps, 5);

        assert_eq!(Color::BLACK, c);
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = default_world();
        w.shapes[0].material_mut().ambient = 1.0;
        *w.shapes[0].material_mut().pattern_mut() =
            Some(Box::new(TestPattern::new()));

        w.shapes[1].material_mut().transparency = 1.0;
        w.shapes[1].material_mut().refractive_index = 1.5;

        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.1), Vector3D::new(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection {
                t: -0.9899,
                object: w.shapes[0].as_ref(),
            },
            Intersection {
                t: -0.4899,
                object: w.shapes[1].as_ref(),
            },
            Intersection {
                t: 0.4899,
                object: w.shapes[1].as_ref(),
            },
            Intersection {
                t: 0.9899,
                object: w.shapes[0].as_ref(),
            },
        ];

        let comps = IntersectionState::new(&xs[2], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(Color::new(0.0, 0.99887, 0.047218), c);
    }

    #[test]
    fn shadow_hit_with_a_transparent_material() {
        let mut w = default_world();

        let mut floor = Plane::new();
        *floor.transform_mut() = Transform::translation(0.0, -1.0, 0.0);
        floor.material_mut().transparency = 0.5;
        floor.material_mut().refractive_index = 1.5;
        w.add_shape(Box::new(floor));

        let mut ball = Sphere::new();
        ball.material_mut().color = Color::new(1.0, 0.0, 0.0);
        ball.material_mut().ambient = 0.5;
        *ball.transform_mut() = Transform::translation(0.0, -3.5, -0.5);
        w.add_shape(Box::new(ball));

        let r = Ray::new(
            Point3D::new(0.0, 0.0, -3.0),
            Vector3D::new(
                0.0,
                -2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
            ),
        );
        let xs = vec![Intersection {
            t: 2f32.sqrt() as FLOAT,
            object: w.shapes[2].as_ref(),
        }];
        let comps = IntersectionState::new(&xs[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);

        assert_eq!(Color::new(0.93642, 0.68642, 0.68642), color);
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -3.0),
            Vector3D::new(
                0.0,
                -2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
            ),
        );

        let mut floor = Plane::new();
        *floor.transform_mut() = Transform::translation(0.0, -1.0, 0.0);
        floor.material_mut().reflective = 0.5;
        floor.material_mut().transparency = 0.5;
        floor.material_mut().refractive_index = 1.5;
        w.add_shape(Box::new(floor));

        let mut ball = Sphere::new();
        ball.material_mut().color = Color::new(1.0, 0.0, 0.0);
        ball.material_mut().ambient = 0.5;
        *ball.transform_mut() = Transform::translation(0.0, -3.5, -0.5);
        w.add_shape(Box::new(ball));

        let xs = vec![Intersection {
            t: 2f32.sqrt() as FLOAT,
            object: w.shapes[2].as_ref(),
        }];
        let comps = IntersectionState::new(&xs[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);

        assert_eq!(Color::new(0.93391, 0.69643, 0.69243), color);
    }
}
