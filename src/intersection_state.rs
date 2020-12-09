use super::{
    intersection::Intersection, point3d::Point3D, ray::Ray, sphere::Sphere,
    vector3d::Vector3D,
};

#[derive(Debug)]
pub struct IntersectionState<'a> {
    pub(crate) t: f32,
    pub(crate) object: &'a Sphere,
    pub(crate) point: Point3D,
    pub(crate) eyev: Vector3D,
    pub(crate) normalv: Vector3D,
    pub(crate) inside: bool,
}

impl<'a> IntersectionState<'a> {
    pub(crate) fn new(i: &'a Intersection, r: &Ray) -> Self {
        let t = i.t;
        let object = i.object;
        let point = r.position(i.t);
        let eyev = -r.direction();
        let mut normalv = object.normal_at(&point);
        let inside = if normalv.dot(&eyev) < 0.0 {
            normalv = -&normalv;
            true
        } else {
            false
        };

        IntersectionState {
            t,
            object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precomputing_the_state_of_intersection() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };

        let comps = IntersectionState::new(&i, &r);
        assert_eq!(i.t, comps.t);
        assert_eq!(Point3D::new(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.normalv);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(
            Point3D::new(0.0, 0.0, -5.0),
            Vector3D::new(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection {
            t: 4.0,
            object: &shape,
        };

        let comps = IntersectionState::new(&i, &r);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection {
            t: 1.0,
            object: &shape,
        };

        let comps = IntersectionState::new(&i, &r);
        assert_eq!(Point3D::new(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(true, comps.inside);
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), comps.normalv);
    }
}
