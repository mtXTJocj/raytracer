use super::{
    intersection::Intersection, node::Node, point3d::Point3D, ray::Ray,
    vector3d::Vector3D, EPSILON, FLOAT,
};

/// 交点における色の計算に必要な情報
pub struct IntersectionState<'a> {
    /// Ray と object が交差する場所での t
    pub(crate) t: FLOAT,
    /// Ray と交差した object
    pub(crate) object: &'a Node,
    /// ワールド座標系における交差位置
    pub(crate) point: Point3D,
    /// self intersection を避けるため point に offset を加えたもの
    /// Shape から出ていく場合用
    pub(crate) over_point: Point3D,
    /// self intersection を避けるため point に offset を加えたもの
    /// Shape 内へ入っていく場合用
    pub(crate) under_point: Point3D,
    /// ワールド座標系における視線ベクトル
    pub(crate) eyev: Vector3D,
    /// ワールド座標系における法線ベクトル
    pub(crate) normalv: Vector3D,
    /// 反射方向のベクトル
    pub(crate) reflectv: Vector3D,
    /// 出射する Shape の屈折率
    pub(crate) n1: FLOAT,
    /// 入射する Shape の屈折率
    pub(crate) n2: FLOAT,
    /// Ray の起点が object 内部であるか
    pub(crate) inside: bool,
}

impl<'a> IntersectionState<'a> {
    /// 新規に IntersectionState を作成する
    ///
    /// # Arguments
    ///
    /// * `i`  - 交点
    /// * `r`  - Ray
    /// * `xs` - r に関する全ての交点
    pub(crate) fn new(
        hit: &'a Intersection,
        r: &Ray,
        xs: &Vec<Intersection>,
    ) -> Self {
        let t = hit.t;
        let object = hit.object;
        let point = r.position(hit.t);
        let eyev = -r.direction();
        let mut normalv = object.normal_at(&point);
        let inside = if normalv.dot(&eyev) < 0.0 {
            normalv = -&normalv;
            true
        } else {
            false
        };
        let over_point = &point + &(&normalv * EPSILON);
        let under_point = &point - &(&normalv * EPSILON);
        let reflectv = r.direction().reflect(&normalv);

        let mut containers: Vec<&Node> = Vec::with_capacity(xs.len());
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for i in xs {
            if std::ptr::eq(i, hit) {
                if let Some(shape) = containers.last() {
                    n1 = shape.material().refractive_index;
                } else {
                    n1 = 1.0;
                }
            }

            if let Some(pos) = containers
                .iter()
                .position(|&shape| std::ptr::eq(shape, i.object))
            {
                // exit shape
                let _ = containers.remove(pos);
            } else {
                // enter shape
                containers.push(i.object);
            }

            if std::ptr::eq(i, hit) {
                if let Some(shape) = containers.last() {
                    n2 = shape.material().refractive_index;
                } else {
                    n2 = 1.0;
                }
            }
        }

        IntersectionState {
            t,
            object,
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            reflectv,
            n1,
            n2,
            inside,
        }
    }

    /// 反射と屈折の割合を計算する
    pub(crate) fn schlick(&self) -> FLOAT {
        let mut cos = self.eyev.dot(&self.normalv);
        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n * n * (1.0 - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt()
        }

        let r0 = (self.n1 - self.n2) / (self.n1 + self.n2);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            approx_eq, plane::Plane, shape::Shape, sphere::Sphere,
            transform::Transform,
        },
        *,
    };

    fn glass_sphere() -> Sphere {
        let mut sphere = Sphere::new();
        sphere.material_mut().transparency = 1.0;
        sphere.material_mut().refractive_index = 1.5;

        sphere
    }

    #[test]
    fn precomputing_the_state_of_intersection() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let node = Node::new(Box::new(Sphere::new()));
        let i = Intersection {
            t: 4.0,
            object: &node,
            u: 0.0,
            v: 0.0,
        };

        let comps = IntersectionState::new(&i, &r, &vec![]);
        assert_eq!(i.t, comps.t);
        assert_eq!(Point3D::new(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.normalv);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let node = Node::new(Box::new(Sphere::new()));
        let i = Intersection {
            t: 4.0,
            object: &node,
            u: 0.0,
            v: 0.0,
        };

        let comps = IntersectionState::new(&i, &r, &vec![]);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let node = Node::new(Box::new(Sphere::new()));
        let i = Intersection {
            t: 1.0,
            object: &node,
            u: 0.0,
            v: 0.0,
        };

        let comps = IntersectionState::new(&i, &r, &vec![]);
        assert_eq!(Point3D::new(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(true, comps.inside);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.normalv);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let node = Node::new(Box::new(Plane::new()));
        let r = Ray::new(
            Point3D::new(0.0, 1.0, -1.0),
            Vector3D::new(
                0.0,
                -2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
            ),
        );
        let i = Intersection {
            t: 2f32.sqrt() as FLOAT,
            object: &node,
            u: 0.0,
            v: 0.0,
        };
        let comps = IntersectionState::new(&i, &r, &vec![]);

        assert_eq!(
            Vector3D::new(
                0.0,
                2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
            ),
            comps.reflectv
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere();
        a.material_mut().refractive_index = 1.5;
        let mut a = Node::new(Box::new(a));
        a.set_transform(Transform::scaling(2.0, 2.0, 2.0));

        let mut b = glass_sphere();
        b.material_mut().refractive_index = 2.0;
        let mut b = Node::new(Box::new(b));
        b.set_transform(Transform::translation(0.0, 0.0, -0.25));

        let mut c = glass_sphere();
        c.material_mut().refractive_index = 2.5;
        let mut c = Node::new(Box::new(c));
        c.set_transform(Transform::translation(0.0, 0.0, 0.25));

        let r = Ray::new(
            Point3D::new(0.0, 0.0, -4.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let xs = vec![
            Intersection {
                t: 2.0,
                object: &a,
                u: 0.0,
                v: 0.0,
            },
            Intersection {
                t: 2.75,
                object: &b,
                u: 0.0,
                v: 0.0,
            },
            Intersection {
                t: 3.25,
                object: &c,
                u: 0.0,
                v: 0.0,
            },
            Intersection {
                t: 4.75,
                object: &b,
                u: 0.0,
                v: 0.0,
            },
            Intersection {
                t: 5.25,
                object: &c,
                u: 0.0,
                v: 0.0,
            },
            Intersection {
                t: 6.0,
                object: &a,
                u: 0.0,
                v: 0.0,
            },
        ];

        let c = IntersectionState::new(&xs[0], &r, &xs);
        assert_eq!(1.0, c.n1);
        assert_eq!(1.5, c.n2);

        let c = IntersectionState::new(&xs[1], &r, &xs);
        assert_eq!(1.5, c.n1);
        assert_eq!(2.0, c.n2);

        let c = IntersectionState::new(&xs[2], &r, &xs);
        assert_eq!(2.0, c.n1);
        assert_eq!(2.5, c.n2);

        let c = IntersectionState::new(&xs[3], &r, &xs);
        assert_eq!(2.5, c.n1);
        assert_eq!(2.5, c.n2);

        let c = IntersectionState::new(&xs[4], &r, &xs);
        assert_eq!(2.5, c.n1);
        assert_eq!(1.5, c.n2);

        let c = IntersectionState::new(&xs[5], &r, &xs);
        assert_eq!(1.5, c.n1);
        assert_eq!(1.0, c.n2);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let mut node = Node::new(Box::new(glass_sphere()));
        node.set_transform(Transform::translation(0.0, 0.0, 1.0));
        let i = Intersection {
            t: 5.0,
            object: &node,
            u: 0.0,
            v: 0.0,
        };
        let xs = vec![i];

        let comps = IntersectionState::new(&xs[0], &r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let node = Node::new(Box::new(glass_sphere()));
        let r = Ray::new(
            Point3D::new(0.0, 0.0, 2f32.sqrt() as FLOAT / 2.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );
        let xs = vec![
            Intersection {
                t: -2f32.sqrt() as FLOAT / 2.0,
                object: &node,
                u: 0.0,
                v: 0.0,
            },
            Intersection {
                t: 2f32.sqrt() as FLOAT / 2.0,
                object: &node,
                u: 0.0,
                v: 0.0,
            },
        ];
        let comps = IntersectionState::new(&xs[1], &r, &xs);
        let reflectance = comps.schlick();
        assert_eq!(1.0, reflectance);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let node = Node::new(Box::new(glass_sphere()));
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection {
                t: -1.0,
                object: &node,
                u: 0.0,
                v: 0.0,
            },
            Intersection {
                t: 1.0,
                object: &node,
                u: 0.0,
                v: 0.0,
            },
        ];
        let comps = IntersectionState::new(&xs[1], &r, &xs);
        let reflectance = comps.schlick();

        assert!(approx_eq(0.04, reflectance));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let node = Node::new(Box::new(glass_sphere()));
        let r = Ray::new(
            Point3D::new(0.0, 0.99, -2.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let xs = vec![Intersection {
            t: 1.8589,
            object: &node,
            u: 0.0,
            v: 0.0,
        }];
        let comps = IntersectionState::new(&xs[0], &r, &xs);
        let reflectance = comps.schlick();

        assert!(approx_eq(0.48873, reflectance));
    }
}
