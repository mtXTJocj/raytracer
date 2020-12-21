use super::{
    intersection::Intersection, point3d::Point3D, ray::Ray, sphere::Sphere,
    vector3d::Vector3D,
};

/// 交点における色の計算に必要な情報
#[derive(Debug)]
pub struct IntersectionState<'a> {
    /// Ray と object が交差する場所での t
    pub(crate) t: f32,
    /// Ray と交差した object
    pub(crate) object: &'a Sphere,
    /// ワールド座標系における交差位置
    pub(crate) point: Point3D,
    /// ワールド座標系における視線ベクトル
    pub(crate) eyev: Vector3D,
    /// ワールド座標系における法線ベクトル
    pub(crate) normalv: Vector3D,
    /// Ray の起点が object 内部であるか
    pub(crate) inside: bool,
}

impl<'a> IntersectionState<'a> {
    /// 新規に IntersectionState を作成する
    ///
    /// # Arguments
    ///
    /// * `i` - 交点
    /// * `r` - Ray
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
