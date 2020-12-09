use super::{point3d::Point3D, sphere::Sphere, vector3d::Vector3D};

#[derive(Debug)]
pub(crate) struct IntersectionState<'a> {
    pub(crate) t: f32,
    pub(crate) object: &'a Sphere,
    pub(crate) point: Point3D,
    pub(crate) eyev: Vector3D,
    pub(crate) normalv: Vector3D,
}
