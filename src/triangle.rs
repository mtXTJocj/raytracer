use crate::{
    intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D, EPSILON,
};

#[derive(Debug)]
pub struct Triangle {
    p1: Point3D,
    p2: Point3D,
    p3: Point3D,
    e1: Vector3D,
    e2: Vector3D,
    normal: Vector3D,
    material: Material,
}

impl Triangle {
    /// 新規に Triangle を作成する
    pub fn new(p1: Point3D, p2: Point3D, p3: Point3D) -> Self {
        let e1 = &p2 - &p1;
        let e2 = &p3 - &p1;
        let mut normal = e1.cross(&e2);
        normal.normalize();
        let material = Material::new();

        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
            material,
        }
    }

    pub fn p1(&self) -> &Point3D {
        &self.p1
    }

    pub fn p2(&self) -> &Point3D {
        &self.p2
    }

    pub fn p3(&self) -> &Point3D {
        &self.p3
    }
}

impl Shape for Triangle {
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
        let dir_cross_e2 = r.direction().cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);
        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = r.origin() - &self.p1;
        let u = f * p1_to_origin.dot(&dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return vec![];
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * r.direction().dot(&origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * self.e2.dot(&origin_cross_e1);
        vec![Intersection {
            t: t,
            object: n,
            u: 0.0,
            v: 0.0,
        }]
    }

    fn local_normal_at(&self, _p: &Point3D, _i: &Intersection) -> Vector3D {
        self.normal.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3d::Vector3D;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Point3D::new(0.0, 1.0, 0.0);
        let p2 = Point3D::new(-1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 0.0, 0.0);
        let t = Triangle::new(p1.clone(), p2.clone(), p3.clone());

        assert_eq!(p1, t.p1);
        assert_eq!(p2, t.p2);
        assert_eq!(p3, t.p3);
        assert_eq!(Vector3D::new(-1.0, -1.0, 0.0), t.e1);
        assert_eq!(Vector3D::new(1.0, -1.0, 0.0), t.e2);
        assert_eq!(Vector3D::new(0.0, 0.0, 1.0), t.normal);
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );
        let i = Intersection {
            t: 0.0,
            object: &Node::new(Box::new(Triangle::new(
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(0.0, 0.0, 0.0),
                Point3D::new(0.0, 0.0, 0.0),
            ))),
            u: 0.0,
            v: 0.0,
        };

        let n1 = t.local_normal_at(&Point3D::new(0.0, 0.5, 0.0), &i);
        let n2 = t.local_normal_at(&Point3D::new(-0.5, 0.75, 0.0), &i);
        let n3 = t.local_normal_at(&Point3D::new(0.5, 0.25, 0.0), &i);

        assert_eq!(t.normal, n1);
        assert_eq!(t.normal, n2);
        assert_eq!(t.normal, n3);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Point3D::new(0.0, -1.0, -2.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );
        let dummy_node = Node::new(Box::new(Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        )));

        let xs = t.local_intersect(&r, &dummy_node);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Point3D::new(1.0, 1.0, -2.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let dummy_node = Node::new(Box::new(Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        )));

        let xs = t.local_intersect(&r, &dummy_node);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Point3D::new(-1.0, 1.0, -2.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let dummy_node = Node::new(Box::new(Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        )));

        let xs = t.local_intersect(&r, &dummy_node);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Point3D::new(0.0, -1.0, -2.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let dummy_node = Node::new(Box::new(Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        )));

        let xs = t.local_intersect(&r, &dummy_node);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(
            Point3D::new(0.0, 0.5, -2.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let dummy_node = Node::new(Box::new(Triangle::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(-1.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        )));

        let xs = t.local_intersect(&r, &dummy_node);

        assert_eq!(1, xs.len());
        assert_eq!(2.0, xs[0].t);
    }
}
