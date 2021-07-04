use super::{
    intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D, EPSILON,
};

#[derive(Debug)]
pub struct Plane {
    /// マテリアル
    material: Material,
}

impl Plane {
    /// 新規に Plane を作成する
    pub fn new() -> Self {
        Plane {
            material: Material::new(),
        }
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_intersect<'a>(
        &self,
        r: &Ray,
        n: &'a Node,
    ) -> Vec<Intersection<'a>> {
        if r.direction().y.abs() < EPSILON {
            return vec![];
        }

        let t = -r.origin().y / r.direction().y;
        vec![Intersection {
            t,
            object: n,
            u: 0.0,
            v: 0.0,
        }]
    }

    fn local_normal_at(&self, _: &Point3D, _: &Intersection) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let i = Intersection {
            t: 0.0,
            object: &Node::new(Box::new(Plane::new())),
            u: 0.0,
            v: 0.0,
        };
        let n1 = p.local_normal_at(&Point3D::new(0.0, 0.0, 0.0), &i);
        let n2 = p.local_normal_at(&Point3D::new(10.0, 0.0, -10.0), &i);
        let n3 = p.local_normal_at(&Point3D::new(-5.0, 0.0, 150.0), &i);

        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n1);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n2);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n3);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let dummy_node = Node::new(Box::new(Plane::new()));

        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, 10.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = p.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let dummy_node = Node::new(Box::new(Plane::new()));

        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, 00.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = p.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let dummy_node = Node::new(Box::new(Plane::new()));

        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, -1.0, 0.0),
        );

        let xs = p.local_intersect(&r, &*dummy_node);
        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert!(std::ptr::eq(xs[0].object, &*dummy_node));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let dummy_node = Node::new(Box::new(Plane::new()));

        let p = Plane::new();
        let r = Ray::new(
            Point3D::new(0.0, -1.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );

        let xs = p.local_intersect(&r, &*dummy_node);
        assert_eq!(1, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert!(std::ptr::eq(xs[0].object, &*dummy_node));
    }
}
