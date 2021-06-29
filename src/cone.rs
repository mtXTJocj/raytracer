use crate::{
    approx_eq, intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D, EPSILON,
    FLOAT, INFINITY,
};

/// Axis Aligned な cube
#[derive(Debug)]
pub struct Cone {
    material: Material,
    ///
    minimum: FLOAT,
    ///
    maximum: FLOAT,
    /// 両端が閉じているか
    closed: bool,
}

impl Cone {
    /// 新規に Cone を作成する
    pub fn new() -> Self {
        Cone {
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
            let y = r.origin().y + t * r.direction().y;
            let z = r.origin().z + t * r.direction().z;

            x * x + z * z <= y * y
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

impl Shape for Cone {
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
        let d = r.direction();
        let o = r.origin();

        let mut xs = vec![];
        let a = d.x * d.x - d.y * d.y + d.z * d.z;
        let b = 2.0 * o.x * d.x - 2.0 * o.y * d.y + 2.0 * o.z * d.z;
        let c = o.x * o.x - o.y * o.y + o.z * o.z;
        if approx_eq(0.0, a) {
            if !approx_eq(0.0, b) {
                let t = -c / (2.0 * b);
                xs.push(Intersection {
                    t: t,
                    object: n,
                    u: 0.0,
                    v: 0.0,
                });
            }
        } else {
            let disc = b * b - 4.0 * a * c;
            if disc >= 0.0 {
                let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
                let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                let y0 = o.y + t0 * d.y;
                if self.minimum() < y0 && y0 < self.maximum() {
                    xs.push(Intersection {
                        t: t0,
                        object: n,
                        u: 0.0,
                        v: 0.0,
                    });
                }
                let y1 = o.y + t1 * d.y;
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
        let mut y = (p.x * p.x + p.z * p.z).sqrt();
        if y < 0.0 {
            y = -y;
        }

        if y < p.y.abs() && p.y >= self.maximum() - EPSILON {
            Vector3D::new(0.0, 1.0, 0.0)
        } else if y < p.y.abs() && p.y <= self.minimum() + EPSILON {
            Vector3D::new(0.0, -1.0, 0.0)
        } else {
            Vector3D::new(p.x, y, p.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{approx_eq, vector3d::Vector3D};

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let dummy_node = Node::new(Box::new(Cone::new()));

        let shape = Cone::new();
        let mut direction = Vector3D::new(0.0, 0.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -5.0), direction);
        let xs = shape.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert!(approx_eq(5.0, xs[0].t));
        assert!(approx_eq(5.0, xs[1].t));

        let mut direction = Vector3D::new(1.0, 1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -5.0), direction);
        let xs = shape.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert!(approx_eq(8.66025, xs[0].t));
        assert!(approx_eq(8.66025, xs[1].t));

        let mut direction = Vector3D::new(-0.5, -1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(1.0, 1.0, -5.0), direction);
        let xs = shape.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert!(approx_eq(4.55006, xs[0].t));
        assert!(approx_eq(49.44994, xs[1].t));
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let dummy_node = Node::new(Box::new(Cone::new()));

        let shape = Box::new(Cone::new());
        let mut direction = Vector3D::new(0.0, 1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -1.0), direction);
        let xs = shape.local_intersect(&r, &dummy_node);
        assert_eq!(1, xs.len());
        assert!(approx_eq(0.35355, xs[0].t));
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let dummy_node = Node::new(Box::new(Cone::new()));

        let mut shape = Cone::new();
        *shape.minimum_mut() = -0.5;
        *shape.maximum_mut() = 0.5;
        *shape.closed_mut() = true;

        let mut direction = Vector3D::new(0.0, 1.0, 0.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -5.0), direction);
        let xs = shape.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let mut direction = Vector3D::new(0.0, 1.0, 1.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -0.25), direction);
        let xs = shape.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());

        let mut direction = Vector3D::new(0.0, 1.0, 0.0);
        direction.normalize();
        let r = Ray::new(Point3D::new(0.0, 0.0, -0.25), direction);
        let xs = shape.local_intersect(&r, &dummy_node);
        assert_eq!(4, xs.len());
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let shape = Cone::new();
        let i = Intersection {
            t: 0.0,
            object: &Node::new(Box::new(Cone::new())),
            u: 0.0,
            v: 0.0,
        };

        let n = shape.local_normal_at(&Point3D::new(0.0, 0.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, 0.0, 0.0), n);

        let n = shape.local_normal_at(&Point3D::new(1.0, 1.0, 1.0), &i);
        assert_eq!(Vector3D::new(1.0, 2f64.sqrt() as FLOAT, 1.0), n);

        let n = shape.local_normal_at(&Point3D::new(-1.0, -1.0, 0.0), &i);
        assert_eq!(Vector3D::new(-1.0, 1.0, 0.0), n);
    }

    #[test]
    fn the_normal_vector_on_a_cones_end_caps() {
        let mut shape = Cone::new();
        *shape.minimum_mut() = -1.0;
        *shape.maximum_mut() = 2.0;
        *shape.closed_mut() = true;
        let i = Intersection {
            t: 0.0,
            object: &Node::new(Box::new(Cone::new())),
            u: 0.0,
            v: 0.0,
        };

        let n = shape.local_normal_at(&Point3D::new(0.0, -1.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, -1.0, 0.0), n);

        let n = shape.local_normal_at(&Point3D::new(0.9, -1.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, -1.0, 0.0), n);

        let n = shape.local_normal_at(&Point3D::new(0.0, -1.0, 0.9), &i);
        assert_eq!(Vector3D::new(0.0, -1.0, 0.0), n);

        let n = shape.local_normal_at(&Point3D::new(0.0, 2.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n);

        let n = shape.local_normal_at(&Point3D::new(1.9, 2.0, 0.0), &i);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n);

        let n = shape.local_normal_at(&Point3D::new(0.0, 2.0, 1.9), &i);
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), n);
    }
}
