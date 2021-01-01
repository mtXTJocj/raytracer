use super::{
    intersection::Intersection, point3d::Point3D, ray::Ray, shape::Shape,
    vector3d::Vector3D, EPSILON, FLOAT,
};

/// 交点における色の計算に必要な情報
pub struct IntersectionState<'a> {
    /// Ray と object が交差する場所での t
    pub(crate) t: FLOAT,
    /// Ray と交差した object
    pub(crate) object: &'a dyn Shape,
    /// ワールド座標系における交差位置
    pub(crate) point: Point3D,
    /// self intersection を避けるため point に offset を加えたもの
    pub(crate) over_point: Point3D,
    pub(crate) under_point: Point3D,
    /// ワールド座標系における視線ベクトル
    pub(crate) eyev: Vector3D,
    /// ワールド座標系における法線ベクトル
    pub(crate) normalv: Vector3D,
    pub(crate) reflectv: Vector3D,
    pub(crate) n1: FLOAT,
    pub(crate) n2: FLOAT,
    /// Ray の起点が object 内部であるか
    pub(crate) inside: bool,
}

impl<'a> IntersectionState<'a> {
    /// 新規に IntersectionState を作成する
    ///
    /// # Arguments
    ///
    /// * `i` - 交点
    /// * `r` - Ray
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

        let mut containers: Vec<&dyn Shape> = Vec::with_capacity(xs.len());
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
}

#[cfg(test)]
mod tests {
    use super::{
        super::{plane::Plane, sphere::Sphere, transform::Transform},
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
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
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
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };

        let comps = IntersectionState::new(&i, &r, &vec![]);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection {
            t: 1.0,
            object: &shape,
        };

        let comps = IntersectionState::new(&i, &r, &vec![]);
        assert_eq!(Point3D::new(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(true, comps.inside);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.normalv);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::new();
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
            object: &shape,
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
        *a.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);
        a.material_mut().refractive_index = 1.5;

        let mut b = glass_sphere();
        *b.transform_mut() = Transform::translation(0.0, 0.0, -0.25);
        b.material_mut().refractive_index = 2.0;

        let mut c = glass_sphere();
        *c.transform_mut() = Transform::translation(0.0, 0.0, 0.25);
        c.material_mut().refractive_index = 2.5;

        let r = Ray::new(
            Point3D::new(0.0, 0.0, -4.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let xs = vec![
            Intersection { t: 2.0, object: &a },
            Intersection {
                t: 2.75,
                object: &b,
            },
            Intersection {
                t: 3.25,
                object: &c,
            },
            Intersection {
                t: 4.75,
                object: &b,
            },
            Intersection {
                t: 5.25,
                object: &c,
            },
            Intersection { t: 6.0, object: &a },
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
        let mut shape = glass_sphere();
        *shape.transform_mut() = Transform::translation(0.0, 0.0, 1.0);
        let i = Intersection {
            t: 5.0,
            object: &shape,
        };
        let xs = vec![i];

        let comps = IntersectionState::new(&xs[0], &r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
}
