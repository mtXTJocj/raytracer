use crate::{
    approx_eq, intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D, EPSILON,
    FLOAT, INFINITY,
};

/// Cylinder
#[derive(Debug)]
pub struct Cylinder {
    material: Material,
    ///
    minimum: FLOAT,
    ///
    maximum: FLOAT,
    /// 両端が閉じているか
    closed: bool,
}

impl Cylinder {
    /// 新規に Cylinder を作成する
    pub fn new() -> Self {
        Cylinder {
            material: Material::new(),
            minimum: -INFINITY,
            maximum: INFINITY,
            closed: false,
        }
    }

    pub fn minimum(&self) -> FLOAT {
        self.minimum
    }

    pub fn minimum_mut(&mut self) -> &mut FLOAT {
        &mut self.minimum
    }

    pub fn maximum(&self) -> FLOAT {
        self.maximum
    }

    pub fn maximum_mut(&mut self) -> &mut FLOAT {
        &mut self.maximum
    }

    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn closed_mut(&mut self) -> &mut bool {
        &mut self.closed
    }

    fn intersect_caps<'a>(
        &'a self,
        r: &Ray,
        n: &'a Node,
        xs: &mut Vec<Intersection<'a>>,
    ) {
        fn check_cap(r: &Ray, t: FLOAT) -> bool {
            let x = r.origin().x + t * r.direction().x;
            let z = r.origin().z + t * r.direction().z;

            x * x + z * z <= 1.0
        }
        if !self.closed() {
            return;
        }
        if approx_eq(0.0, r.direction().y) {
            return;
        }

        // check for an intersection with the lower end cap
        // by intersecting the ray with the plane at y = cyl.minimum
        let t = (self.minimum() - r.origin().y) / r.direction().y;
        if check_cap(&r, t) {
            xs.push(Intersection {
                t: t,
                object: n,
                u: 0.0,
                v: 0.0,
            });
        }

        // check for an intersection with the upper end cap
        // by intersecting the ray with the plane at y = cyl.maximum
        let t = (self.maximum() - r.origin().y) / r.direction().y;
        if check_cap(&r, t) {
            xs.push(Intersection {
                t: t,
                object: n,
                u: 0.0,
                v: 0.0,
            });
        }
    }
}

impl Shape for Cylinder {
    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_intersect<'a>(
        &'a self,
        r: &Ray,
        n: &'a Node,
    ) -> Vec<Intersection<'a>> {
        let dir = r.direction();
        let o = r.origin();

        let mut xs = vec![];
        let a = dir.x * dir.x + dir.z * dir.z;
        if !approx_eq(0.0, a) {
            let b = 2.0 * o.x * dir.x + 2.0 * o.z * dir.z;
            let c = o.x * o.x + o.z * o.z - 1.0;
            let disc = b * b - 4.0 * a * c;
            if disc >= 0.0 {
                let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
                let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                let y0 = o.y + t0 * dir.y;
                if self.minimum() < y0 && y0 < self.maximum() {
                    xs.push(Intersection {
                        t: t0,
                        object: n,
                        u: 0.0,
                        v: 0.0,
                    });
                }
                let y1 = o.y + t1 * dir.y;
                if self.minimum() < y1 && y1 < self.maximum() {
                    xs.push(Intersection {
                        t: t1,
                        object: n,
                        u: 0.0,
                        v: 0.0,
                    });
                }
            }
        }

        self.intersect_caps(&r, n, &mut xs);
        xs
    }

    fn local_normal_at(&self, p: &Point3D, _: &Intersection) -> Vector3D {
        let dist = p.x * p.x + p.z * p.z;

        if dist < 1.0 && p.y >= self.maximum() - EPSILON {
            Vector3D::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && p.y <= self.minimum() + EPSILON {
            Vector3D::new(0.0, -1.0, 0.0)
        } else {
            Vector3D::new(p.x, 0.0, p.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{approx_eq, vector3d::Vector3D};

    #[test]
    fn a_ray_misses_a_cylinder() {
        let dummy_node = Node::new(Box::new(Cylinder::new()));

        let cyl = Cylinder::new();
        let mut direction = Vector3D::new(0.0, 1.0, 0.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(1.0, 0.0, 0.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(0.0, 1.0, 0.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, 0.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(1.0, 1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let dummy_node = Node::new(Box::new(Cylinder::new()));

        let cyl = Cylinder::new();
        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(1.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);

        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);

        let mut direction = Vector3D::new(0.1, 1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.5, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert!(approx_eq(6.80798, xs[0].t));
        assert!(approx_eq(7.08872, xs[1].t));
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Cylinder::new();
        let i = Intersection {
            t: 0.0,
            object: &Node::new(Box::new(Cylinder::new())),
            u: 0.0,
            v: 0.0,
        };

        let n = cyl.local_normal_at(&Point3D::new(1.0, 0.0, 0.0), &i);
        assert_eq!(Vector3D::new(1.0, 0.0, 0.0), n);

        let n = cyl.local_normal_at(&Point3D::new(0.0, 5.0, -1.0), &i);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), n);

        let n = cyl.local_normal_at(&Point3D::new(0.0, -2.0, 1.0), &i);
        assert_eq!(Vector3D::new(0.0, 0.0, 1.0), n);

        let n = cyl.local_normal_at(&Point3D::new(-1.0, 1.0, 0.0), &i);
        assert_eq!(Vector3D::new(-1.0, 0.0, 0.0), n);
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(-INFINITY, cyl.minimum());
        assert_eq!(INFINITY, cyl.maximum());
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert_eq!(false, cyl.closed());
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let dummy_node = Node::new(Box::new(Cylinder::new()));

        let mut cyl = Cylinder::new();
        *cyl.minimum_mut() = 1.0;
        *cyl.maximum_mut() = 2.0;

        let mut direction = Vector3D::new(0.1, 1.0, 0.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 1.5, 0.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 3.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 2.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 1.0, -5.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 1.5, -2.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let dummy_node = Node::new(Box::new(Cylinder::new()));

        let mut cyl = Cylinder::new();
        *cyl.minimum_mut() = 1.0;
        *cyl.maximum_mut() = 2.0;
        *cyl.closed_mut() = true;

        let mut direction = Vector3D::new(0.0, -1.0, 0.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 3.0, 0.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());

        let mut direction = Vector3D::new(0.0, -1.0, 2.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 3.0, -2.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());

        let mut direction = Vector3D::new(0.0, -1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 4.0, -2.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());

        let mut direction = Vector3D::new(0.0, 1.0, 2.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -2.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());

        let mut direction = Vector3D::new(0.0, 1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, -1.0, -2.0), direction);
        let xs = cyl.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let mut cyl = Cylinder::new();
        *cyl.minimum_mut() = 1.0;
        *cyl.maximum_mut() = 2.0;
        *cyl.closed_mut() = true;
        let i = Intersection {
            t: 0.0,
            object: &Node::new(Box::new(Cylinder::new())),
            u: 0.0,
            v: 0.0,
        };

        let n = cyl.local_normal_at(&Point3D::new(0.0, 1.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, -1.0, 0.0), n);

        let n = cyl.local_normal_at(&Point3D::new(0.5, 1.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, -1.0, 0.0), n);

        let n = cyl.local_normal_at(&Point3D::new(0.0, 1.0, 0.5), &i);
        assert_eq!(Vector3D::new(0.0, -1.0, 0.0), n);

        let n = cyl.local_normal_at(&Point3D::new(0.0, 2.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n);

        let n = cyl.local_normal_at(&Point3D::new(0.5, 2.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n);

        let n = cyl.local_normal_at(&Point3D::new(0.0, 2.0, 0.5), &i);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n);
    }
}
