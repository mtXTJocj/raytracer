use super::{
    intersection::Intersection, material::Material, point3d::Point3D, ray::Ray,
    shape::Shape, transform::Transform, vector3d::Vector3D, EPSILON,
};

#[derive(Debug)]
pub struct Plane {
    /// Local-World 変換
    transform: Transform,
    /// マテリアル
    material: Material,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: Transform::identity(),
            material: Material::new(),
        }
    }
}

impl Shape for Plane {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_intersect(&self, r: &Ray) -> Vec<Intersection> {
        if r.direction().y.abs() < EPSILON {
            return vec![];
        }

        let t = -r.origin().y / r.direction().y;
        vec![Intersection { t, object: self }]
    }

    fn local_normal_at(&self, _: &Point3D) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(&Point3D::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&Point3D::new(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&Point3D::new(-5.0, 0.0, 150.0));

        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n1);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n2);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n3);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, 10.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = p.local_intersect(&r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, 00.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = p.local_intersect(&r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, -1.0, 0.0),
        );

        let xs = p.local_intersect(&r);
        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert!(std::ptr::eq(
            xs[0].object as *const _ as *const (),
            &p as *const _ as *const ()
        ));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, -1.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        let xs = p.local_intersect(&r);
        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert!(std::ptr::eq(
            xs[0].object as *const _ as *const (),
            &p as *const _ as *const ()
        ));
    }
}
