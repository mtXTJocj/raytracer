use super::{
    intersection::Intersection, material::Material, point3d::Point3D, ray::Ray,
    transform::Transform, vector3d::Vector3D,
};

/// 原点を中心とする半径 1 の単位球
#[derive(Debug)]
pub struct Sphere {
    /// 球に対して適用する変換
    transform: Transform,
    material: Material,
}

impl Sphere {
    /// 新規に Sphere を作成する
    pub fn new() -> Self {
        Sphere {
            transform: Transform::identity(),
            material: Material::new(),
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

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
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

    /// self 上の点 p における法線ベクトルを取得する。
    ///
    /// # Argumets
    /// * `p` - self 上の点
    pub fn normal_at(&self, p: &Point3D) -> Vector3D {
        let p_in_local = self.transform.inv() * p;
        let n = Vector3D::new(p_in_local.x, p_in_local.y, p_in_local.z);

        self.transform.apply_to_normal(&n)
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

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Point3D::new(1.0, 0.0, 0.0));

        assert_eq!(Vector3D::new(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Point3D::new(0.0, 1.0, 0.0));

        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&Point3D::new(0.0, 0.0, 1.0));

        assert_eq!(Vector3D::new(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(&Point3D::new(
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
        ));

        assert_eq!(
            Vector3D::new(
                3f32.sqrt() / 3.0,
                3f32.sqrt() / 3.0,
                3f32.sqrt() / 3.0,
            ),
            n
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let mut n = s.normal_at(&Point3D::new(
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
            3f32.sqrt() / 3.0,
        ));

        assert_eq!(
            s.normal_at(&Point3D::new(
                3f32.sqrt() / 3.0,
                3f32.sqrt() / 3.0,
                3f32.sqrt() / 3.0
            )),
            *n.normalize()
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        *s.transform_mut() = Transform::translation(0.0, 1.0, 0.0);

        let n = s.normal_at(&Point3D::new(0.0, 1.70711, -0.70711));
        assert_eq!(Vector3D::new(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        *s.transform_mut() = &Transform::scaling(1.0, 0.5, 1.0)
            * &Transform::rotation_z(std::f32::consts::PI / 5.0);

        let n = s.normal_at(&Point3D::new(
            0.0,
            2f32.sqrt() / 2.0,
            -2f32.sqrt() / 2.0,
        ));
        assert_eq!(Vector3D::new(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = Material::new();

        assert_eq!(m.color, s.material().color);
        assert_eq!(m.ambient, s.material().ambient);
        assert_eq!(m.diffuse, s.material().diffuse);
        assert_eq!(m.specular, s.material().specular);
        assert_eq!(m.shininess, s.material().shininess);
    }

    #[test]
    fn a_shpere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;

        *s.material_mut() = m;
        assert_eq!(1.0, s.material().ambient);
    }
}
