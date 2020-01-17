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
}
