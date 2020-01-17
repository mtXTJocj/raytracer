use super::{point3d::Point3D, vector3d::Vector3D};

pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3D {
        &self.direction
    }

    pub fn position(&self, t: f32) -> Point3D {
        &self.origin + &(t * &self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point3D::new(1.0, 2.0, 3.0);
        let direction = Vector3D::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin.clone(), direction.clone());

        assert_eq!(origin, *r.origin());
        assert_eq!(direction, *r.direction());
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let ray =
            Ray::new(Point3D::new(2.0, 3.0, 4.0), Vector3D::new(1.0, 0.0, 0.0));

        assert_eq!(Point3D::new(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Point3D::new(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Point3D::new(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Point3D::new(4.5, 3.0, 4.0), ray.position(2.5));
    }
}
