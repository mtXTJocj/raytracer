use super::{intersection::Intersection, point3d::Point3D, ray::Ray};

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Sphere {}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let o = ray.origin();
        let d = ray.direction();
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
}
