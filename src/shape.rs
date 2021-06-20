use super::{
    intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, vector3d::Vector3D,
};
use std::fmt::Debug;

pub trait Shape: Debug {
    fn add_child(&mut self, child: Box<Node>) {
        panic!();
    }

    /// Material を取得する
    fn material(&self) -> &Material;
    /// Material を取得する
    fn material_mut(&mut self) -> &mut Material;

    /// 各 Shape 固有の衝突判定を行う
    ///
    /// # Argumets
    /// * `ray` - local 座標系における Ray
    fn local_intersect<'a>(
        &'a self,
        r: &Ray,
        n: &'a Node,
    ) -> Vec<Intersection<'a>>;

    /// local 座標上の点 p における法線ベクトルを取得する。
    ///
    /// # Argumets
    /// * `p` - local 座標系上の点
    fn local_normal_at(&self, p: &Point3D) -> Vector3D;
}

#[cfg(test)]
mod tests {
    use super::{
        super::{color::Color, transform::Transform, FLOAT},
        *,
    };

    #[derive(Debug)]
    struct TestShape {
        material: Material,
        local_ray: Ray,
    }

    impl TestShape {
        fn new(r: Ray) -> Self {
            TestShape {
                material: Material::new(),
                local_ray: r,
            }
        }
    }

    impl Shape for TestShape {
        fn material(&self) -> &Material {
            &self.material
        }

        fn material_mut(&mut self) -> &mut Material {
            &mut self.material
        }

        fn local_intersect(&self, r: &Ray, _: &Node) -> Vec<Intersection> {
            assert_eq!(self.local_ray.origin(), r.origin());
            assert_eq!(self.local_ray.direction(), r.direction());

            vec![]
        }

        fn local_normal_at(&self, p: &Point3D) -> Vector3D {
            Vector3D::new(p.x, p.y, p.z)
        }
    }

    fn test_shape() -> impl Shape {
        TestShape::new(Ray::new(
            Point3D::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
        ))
    }

    #[test]
    fn the_default_material() {
        let shape = test_shape();

        assert_eq!(Color::WHITE, shape.material().color);
        assert_eq!(0.1, shape.material().ambient);
        assert_eq!(0.9, shape.material().diffuse);
        assert_eq!(0.9, shape.material().specular);
        assert_eq!(200.0, shape.material().shininess);
    }

    #[test]
    fn assigning_a_material() {
        let mut shape = test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;

        *shape.material_mut() = m;
        assert_eq!(1.0, shape.material().ambient);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let mut node = Node::new(Box::new(TestShape::new(Ray::new(
            Point3D::new(0.0, 0.0, -2.5),
            Vector3D::new(0.0, 0.0, 0.5),
        ))));
        node.set_transform(Transform::scaling(2.0, 2.0, 2.0));

        let xs = node.intersect(&r);
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let mut node = Node::new(Box::new(TestShape::new(Ray::new(
            Point3D::new(-5.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        ))));
        node.set_transform(Transform::translation(5.0, 0.0, 0.0));

        let xs = node.intersect(&r);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = Node::new(Box::new(test_shape()));
        s.set_transform(Transform::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Point3D::new(0.0, 1.70711, -0.70711));

        assert_eq!(Vector3D::new(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = Node::new(Box::new(test_shape()));
        s.set_transform(
            &Transform::scaling(1.0, 0.5, 1.0)
                * &Transform::rotation_z(std::f32::consts::PI as FLOAT / 5.0),
        );
        let n = s.normal_at(&Point3D::new(
            0.0,
            2f32.sqrt() as FLOAT / 2.0,
            -2f32.sqrt() as FLOAT / 2.0,
        ));

        assert_eq!(Vector3D::new(0.0, 0.97014, -0.24254), n);
    }
}
