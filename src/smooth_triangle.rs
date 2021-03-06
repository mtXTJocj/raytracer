use crate::{
    intersection::Intersection, material::Material, node::Node,
    point3d::Point3D, ray::Ray, shape::Shape, vector3d::Vector3D, EPSILON,
};

#[derive(Debug)]
pub struct SmoothTriangle {
    p1: Point3D,
    p2: Point3D,
    p3: Point3D,
    n1: Vector3D,
    n2: Vector3D,
    n3: Vector3D,
    e1: Vector3D,
    e2: Vector3D,
    normal: Vector3D,
    material: Material,
}

impl SmoothTriangle {
    /// 新規に SmoothTriangle を作成する
    pub fn new(
        p1: Point3D,
        p2: Point3D,
        p3: Point3D,
        n1: Vector3D,
        n2: Vector3D,
        n3: Vector3D,
    ) -> Self {
        let e1 = &p2 - &p1;
        let e2 = &p3 - &p1;
        let mut normal = e1.cross(&e2);
        normal.normalize();
        let material = Material::new();

        SmoothTriangle {
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
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

    pub fn n1(&self) -> &Vector3D {
        &self.n1
    }

    pub fn n2(&self) -> &Vector3D {
        &self.n2
    }

    pub fn n3(&self) -> &Vector3D {
        &self.n3
    }
}

impl Shape for SmoothTriangle {
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
            u,
            v,
        }]
    }

    fn local_normal_at(&self, _: &Point3D, i: &Intersection) -> Vector3D {
        &(&(&self.n2 * i.u) + &(&self.n3 * i.v))
            + &(&self.n1 * (1.0 - i.u - i.v))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{approx_eq, intersection_state::IntersectionState},
        *,
    };
    use crate::vector3d::Vector3D;

    #[test]
    fn constructing_a_smooth_triangle() {
        let p1 = Point3D::new(0.0, 1.0, 0.0);
        let p2 = Point3D::new(-1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 0.0, 0.0);
        let n1 = Vector3D::new(0.0, 1.0, 0.0);
        let n2 = Vector3D::new(-1.0, 0.0, 0.0);
        let n3 = Vector3D::new(1.0, 0.0, 0.0);

        let t = SmoothTriangle::new(
            p1.clone(),
            p2.clone(),
            p3.clone(),
            n1.clone(),
            n2.clone(),
            n3.clone(),
        );

        assert_eq!(p1, *t.p1());
        assert_eq!(p2, *t.p2());
        assert_eq!(p3, *t.p3());
        assert_eq!(n1, *t.n1());
        assert_eq!(n2, *t.n2());
        assert_eq!(n3, *t.n3());
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_uv() {
        let p1 = Point3D::new(0.0, 1.0, 0.0);
        let p2 = Point3D::new(-1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 0.0, 0.0);
        let n1 = Vector3D::new(0.0, 1.0, 0.0);
        let n2 = Vector3D::new(-1.0, 0.0, 0.0);
        let n3 = Vector3D::new(1.0, 0.0, 0.0);

        let tri = SmoothTriangle::new(
            p1.clone(),
            p2.clone(),
            p3.clone(),
            n1.clone(),
            n2.clone(),
            n3.clone(),
        );
        let dummy_node =
            Node::new(Box::new(SmoothTriangle::new(p1, p2, p3, n1, n2, n3)));

        let r = Ray::new(
            Point3D::new(-0.2, 0.3, -2.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );

        let xs = tri.local_intersect(&r, &dummy_node);

        assert!(approx_eq(0.45, xs[0].u));
        assert!(approx_eq(0.25, xs[0].v));
    }

    #[test]
    fn a_smooth_triangle_uses_uv_to_interpolate_the_normal() {
        let p1 = Point3D::new(0.0, 1.0, 0.0);
        let p2 = Point3D::new(-1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 0.0, 0.0);
        let n1 = Vector3D::new(0.0, 1.0, 0.0);
        let n2 = Vector3D::new(-1.0, 0.0, 0.0);
        let n3 = Vector3D::new(1.0, 0.0, 0.0);

        let tri =
            Node::new(Box::new(SmoothTriangle::new(p1, p2, p3, n1, n2, n3)));
        let i = Intersection {
            t: 1.0,
            object: &tri,
            u: 0.45,
            v: 0.25,
        };

        let n = tri.normal_at(&Point3D::new(0.0, 0.0, 0.0), &i);
        assert_eq!(Vector3D::new(-0.5547, 0.83205, 0.0), n)
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let p1 = Point3D::new(0.0, 1.0, 0.0);
        let p2 = Point3D::new(-1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 0.0, 0.0);
        let n1 = Vector3D::new(0.0, 1.0, 0.0);
        let n2 = Vector3D::new(-1.0, 0.0, 0.0);
        let n3 = Vector3D::new(1.0, 0.0, 0.0);
        let tri =
            Node::new(Box::new(SmoothTriangle::new(p1, p2, p3, n1, n2, n3)));

        let r = Ray::new(
            Point3D::new(-0.2, 0.3, -2.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let i = Intersection {
            t: 1.0,
            object: &tri,
            u: 0.45,
            v: 0.25,
        };
        let xs = vec![i];
        let comps = IntersectionState::new(&xs[0], &r, &xs);
        assert_eq!(Vector3D::new(-0.5547, 0.83205, 0.0), comps.normalv);
    }
}
