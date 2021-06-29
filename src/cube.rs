use super::{
    intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D, EPSILON,
    FLOAT, INFINITY,
};

/// Axis Aligned な cube
#[derive(Debug)]
pub struct Cube {
    material: Material,
}

impl Cube {
    /// 新規に Cube を作成する
    /// Cube は中心を原点とする Axis-Aligned Box で、各軸 1, -1 に面が存在する
    pub fn new() -> Self {
        Cube {
            material: Material::new(),
        }
    }
}

impl Shape for Cube {
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
        /// Ray の各軸の面との交点となる t を求める。
        ///
        /// # Argumets
        /// * `origin` - Ray の開始点
        /// * `direction` - Ray の方向
        fn check_axis(origin: FLOAT, direction: FLOAT) -> (FLOAT, FLOAT) {
            // -1 の面
            let tmin_numerator = -1.0 - origin;
            // 1 の面
            let tmax_numerator = 1.0 - origin;

            let tmin;
            let tmax;
            if direction.abs() >= EPSILON {
                tmin = tmin_numerator / direction;
                tmax = tmax_numerator / direction;
            } else {
                tmin = tmin_numerator * INFINITY;
                tmax = tmax_numerator * INFINITY;
            }

            if tmin > tmax {
                (tmax, tmin)
            } else {
                (tmin, tmax)
            }
        }

        let (xtmin, xtmax) = check_axis(r.origin().x, r.direction().x);
        let (ytmin, ytmax) = check_axis(r.origin().y, r.direction().y);
        let (ztmin, ztmax) = check_axis(r.origin().z, r.direction().z);

        // largest minimum
        let tmin = xtmin.max(ytmin).max(ztmin);
        // smallest maximum
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![
                Intersection {
                    t: tmin,
                    object: n,
                    u: 0.0,
                    v: 0.0,
                },
                Intersection {
                    t: tmax,
                    object: n,
                    u: 0.0,
                    v: 0.0,
                },
            ]
        }
    }

    fn local_normal_at(&self, p: &Point3D) -> Vector3D {
        let maxc = p.x.abs().max(p.y.abs()).max(p.z.abs());

        if maxc == p.x.abs() {
            Vector3D::new(p.x, 0.0, 0.0)
        } else if maxc == p.y.abs() {
            Vector3D::new(0.0, p.y, 0.0)
        } else {
            Vector3D::new(0.0, 0.0, p.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::approx_eq, super::vector3d::Vector3D, *};

    #[test]
    fn a_ray_intersects_a_cube() {
        let dummy_node = Node::new(Box::new(Cube::new()));

        let c = Cube::new();

        // +x
        let origin = Point3D::new(5.0, 0.5, 0.0);
        let direction = Vector3D::new(-1.0, 0.0, 0.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);

        // -x
        let origin = Point3D::new(-5.0, 0.5, 0.0);
        let direction = Vector3D::new(1.0, 0.0, 0.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);

        // +y
        let origin = Point3D::new(0.5, 5.0, 0.0);
        let direction = Vector3D::new(0.0, -1.0, 0.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);

        // -y
        let origin = Point3D::new(0.5, -5.0, 0.0);
        let direction = Vector3D::new(0.0, 1.0, 0.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);

        // +z
        let origin = Point3D::new(0.5, 0.0, 5.0);
        let direction = Vector3D::new(0.0, 0.0, -1.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);

        // -z
        let origin = Point3D::new(0.5, 0.0, -5.0);
        let direction = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);

        // inside
        let origin = Point3D::new(0.0, 0.5, 0.0);
        let direction = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let dummy_node = Node::new(Box::new(Cube::new()));

        let c = Cube::new();

        let origin = Point3D::new(-2.0, 0.0, 0.0);
        let direction = Vector3D::new(0.2673, 0.5345, 0.8018);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let origin = Point3D::new(0.0, -2.0, 0.0);
        let direction = Vector3D::new(0.8018, 0.2673, 0.5345);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let origin = Point3D::new(0.0, 0.0, -2.0);
        let direction = Vector3D::new(0.5345, 0.8018, 0.2673);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let origin = Point3D::new(2.0, 0.0, 2.0);
        let direction = Vector3D::new(0.0, 0.0, -1.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let origin = Point3D::new(0.0, 2.0, 2.0);
        let direction = Vector3D::new(0.0, -1.0, 0.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());

        let origin = Point3D::new(2.0, 2.0, 0.0);
        let direction = Vector3D::new(-1.0, 0.0, 0.0);
        let r = Ray::new(origin, direction);
        let xs = c.local_intersect(&r, &dummy_node);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::new();

        let p = Point3D::new(1.0, 0.5, -0.8);
        let n = Vector3D::new(1.0, 0.0, 0.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);

        let p = Point3D::new(-1.0, -0.2, 0.9);
        let n = Vector3D::new(-1.0, 0.0, 0.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);

        let p = Point3D::new(-0.4, 1.0, -0.1);
        let n = Vector3D::new(0.0, 1.0, 0.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);

        let p = Point3D::new(0.3, -1.0, -0.7);
        let n = Vector3D::new(0.0, -1.0, 0.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);

        let p = Point3D::new(-0.6, 0.3, 1.0);
        let n = Vector3D::new(0.0, 0.0, 1.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);

        let p = Point3D::new(0.4, 0.4, -1.0);
        let n = Vector3D::new(0.0, 0.0, -1.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);

        let p = Point3D::new(1.0, 1.0, 1.0);
        let n = Vector3D::new(1.0, 0.0, 0.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);

        let p = Point3D::new(-1.0, -1.0, -1.0);
        let n = Vector3D::new(-1.0, 0.0, 0.0);
        let normal = c.local_normal_at(&p);
        assert_eq!(n, normal);
    }
}
