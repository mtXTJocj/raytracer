use super::{point3d::Point3D, vector3d::Vector3D, FLOAT};

/// Ray
#[derive(Debug)]
pub struct Ray {
    /// Ray の始点
    origin: Point3D,
    /// Ray の方向
    direction: Vector3D,
}

impl Ray {
    /// 新規に Ray を作成する
    ///
    /// # Argumets
    /// * `origin` - Ray の始点
    /// * `direction` - Ray の方向
    pub fn new(origin: Point3D, direction: Vector3D) -> Self {
        Ray { origin, direction }
    }

    /// Ray の始点を取得する
    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    /// Ray の方向を取得する
    pub fn direction(&self) -> &Vector3D {
        &self.direction
    }

    /// origin から direction 方向に t だけ進んだ点を取得する
    ///
    /// # Argumets
    /// * `t` - direction 方向の距離を示すパラメータ
    pub fn position(&self, t: FLOAT) -> Point3D {
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
