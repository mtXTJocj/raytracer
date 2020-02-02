use super::{
    intersection::Intersection, point3d::Point3D, ray::Ray,
    transform::Transform,
};

/// 原点を中心とする半径 1 の単位球
#[derive(Debug)]
pub struct Sphere {
    /// 球に対して適用する変換
    transform: Transform,
}

impl Sphere {
    /// 新規に Sphere を作成する
    pub fn new() -> Self {
        Sphere {
            transform: Transform::identity(),
        }
    }

    /// self に対する変換を取得する
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    /// self に対する変換を取得する
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    /// ray と self の交点を求める。全ての交点を Vec に入れて返す。
    /// 交点がない場合には空の Vec を返す。
    ///
    /// # Argumets
    /// * `ray` - 交点の計算対象となる Ray
    ///
    /// # Returns
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let r = self.transform.inv() * ray;
        let o = r.origin();
        let d = r.direction();
        let sphere_to_ray = o - &Point3D::ZERO;

        let a = d.dot(&d);
        let b = 2.0 * d.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // no intersection
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        return vec![
            Intersection {
                t: t1,
                object: self,
            },
            Intersection {
                t: t2,
                object: self,
            },
        ];
    }
}

#[cfg(test)]
mod tests {
    use super::{super::approx_eq, super::vector3d::Vector3D, *};

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());

        assert!(approx_eq(xs[0].t, 4.0));
        assert!(approx_eq(xs[1].t, 6.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            Point3D::new(0.0, 1.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());

        assert!(approx_eq(xs[0].t, 5.0));
        assert!(approx_eq(xs[1].t, 5.0));
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());

        assert!(approx_eq(xs[0].t, -1.0));
        assert!(approx_eq(xs[1].t, 1.0));
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(&r);
        assert_eq!(2, xs.len());

        assert!(approx_eq(xs[0].t, -6.0));
        assert!(approx_eq(xs[1].t, -4.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersecion() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(2, xs.len());
        assert!(std::ptr::eq(xs[0].object, &s));
        assert!(std::ptr::eq(xs[1].object, &s));
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(Transform::identity(), *s.transform());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::new();
        let x = 2.0;
        let y = 3.0;
        let z = 4.0;
        let t = Transform::translation(x, y, z);
        *s.transform_mut() = t;

        assert_eq!(Transform::translation(x, y, z), *s.transform());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        *s.transform_mut() = Transform::scaling(2.0, 2.0, 2.0);

        let xs = s.intersect(&r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let mut s = Sphere::new();
        *s.transform_mut() = Transform::translation(5.0, 0.0, 0.0);

        let xs = s.intersect(&r);

        assert_eq!(0, xs.len());
    }
}
