use super::{matrix4x4::Matrix4x4, point3d::Point3D, vector3d::Vector3D};
use std::ops::Mul;

pub struct Transform {
    mat: Matrix4x4,
    inv: Matrix4x4,
}

impl Transform {
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mat = Matrix4x4::new([
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0,
            0.0, 1.0,
        ]);
        let inv = Matrix4x4::new([
            1.0, 0.0, 0.0, -x, 0.0, 1.0, 0.0, -y, 0.0, 0.0, 1.0, -z, 0.0, 0.0,
            0.0, 1.0,
        ]);
        Transform { mat, inv }
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        assert!(x != 0.0);
        assert!(y != 0.0);
        assert!(z != 0.0);

        let mat = Matrix4x4::new([
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0,
            0.0, 1.0,
        ]);
        let inv = Matrix4x4::new([
            1.0 / x,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / y,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);
        Transform { mat, inv }
    }

    pub fn rotation_x(a: f32) -> Self {
        let mat = Matrix4x4::new([
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            a.cos(),
            -a.sin(),
            0.0,
            0.0,
            a.sin(),
            a.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);
        let inv = mat.transpose();
        Transform { mat, inv }
    }

    pub fn rotation_y(a: f32) -> Self {
        let mat = Matrix4x4::new([
            a.cos(),
            0.0,
            a.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            -a.sin(),
            0.0,
            a.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);
        let inv = mat.transpose();
        Transform { mat, inv }
    }

    pub fn rotation_z(a: f32) -> Self {
        let mat = Matrix4x4::new([
            a.cos(),
            -a.sin(),
            0.0,
            0.0,
            a.sin(),
            a.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);
        let inv = mat.transpose();
        Transform { mat, inv }
    }

    pub fn shearing(
        xy: f32,
        xz: f32,
        yx: f32,
        yz: f32,
        zx: f32,
        zy: f32,
    ) -> Self {
        let mat = Matrix4x4::new([
            1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0,
            0.0, 1.0,
        ]);
        let inv = mat.inverse();
        Transform { mat, inv }
    }

    pub fn inv(&self) -> &Matrix4x4 {
        &self.inv
    }
}

impl Mul<&Point3D> for &Transform {
    type Output = Point3D;

    fn mul(self, x: &Point3D) -> Self::Output {
        &self.mat * x
    }
}

impl Mul<&Vector3D> for &Transform {
    type Output = Vector3D;

    fn mul(self, x: &Vector3D) -> Self::Output {
        &self.mat * x
    }
}

impl Mul<&Transform> for &Transform {
    type Output = Transform;

    fn mul(self, t: &Transform) -> Self::Output {
        Transform {
            mat: &self.mat * &t.mat,
            inv: &t.inv * &self.inv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let t = Transform::translation(5.0, -3.0, 2.0);
        let p = Point3D::new(-3.0, 4.0, 5.0);

        assert_eq!(Point3D::new(2.0, 1.0, 7.0), &t * &p);
    }

    #[test]
    fn multiplying_by_the_inverse_of_tanslation_matrix() {
        let t = Transform::translation(5.0, -3.0, 2.0);
        let p = Point3D::new(-3.0, 4.0, 5.0);

        assert_eq!(Point3D::new(-8.0, 7.0, 3.0), t.inv() * &p);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = Transform::translation(5.0, -3.0, 2.0);
        let v = Vector3D::new(-3.0, 4.0, 5.0);

        assert_eq!(v, &t * &v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let t = Transform::scaling(2.0, 3.0, 4.0);
        let p = Point3D::new(-4.0, 6.0, 8.0);

        assert_eq!(Point3D::new(-8.0, 18.0, 32.0), &t * &p);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let t = Transform::scaling(2.0, 3.0, 4.0);
        let v = Vector3D::new(-4.0, 6.0, 8.0);

        assert_eq!(Vector3D::new(-8.0, 18.0, 32.0), &t * &v);
    }

    #[test]
    fn multiplying_by_the_inverse_of_scaling_matrix() {
        let t = Transform::scaling(2.0, 3.0, 4.0);
        let v = Vector3D::new(-4.0, 6.0, 8.0);

        assert_eq!(Vector3D::new(-2.0, 2.0, 2.0), t.inv() * &v);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let t = Transform::scaling(-1.0, 1.0, 1.0);
        let p = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(Point3D::new(-2.0, 3.0, 4.0), &t * &p);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_x(std::f32::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_x(std::f32::consts::FRAC_PI_2);

        assert_eq!(
            Point3D::new(0.0, 2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0),
            &half_quarter * &p
        );
        assert_eq!(Point3D::new(0.0, 0.0, 1.0), &full_quarter * &p);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_x(std::f32::consts::FRAC_PI_4);

        assert_eq!(
            Point3D::new(0.0, 2f32.sqrt() / 2.0, -2f32.sqrt() / 2.0),
            half_quarter.inv() * &p
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Point3D::new(0.0, 0.0, 1.0);
        let half_quarter = Transform::rotation_y(std::f32::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_y(std::f32::consts::FRAC_PI_2);

        assert_eq!(
            Point3D::new(2f32.sqrt() / 2.0, 0.0, 2f32.sqrt() / 2.0),
            &half_quarter * &p
        );
        assert_eq!(Point3D::new(1.0, 0.0, 0.0), &full_quarter * &p);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_z(std::f32::consts::FRAC_PI_4);
        let full_quarter = Transform::rotation_z(std::f32::consts::FRAC_PI_2);

        assert_eq!(
            Point3D::new(-2f32.sqrt() / 2.0, 2f32.sqrt() / 2.0, 0.0),
            &half_quarter * &p
        );
        assert_eq!(Point3D::new(-1.0, 0.0, 0.0), &full_quarter * &p);
    }

    #[test]
    fn a_shearing_information_moves_x_in_propotion_to_y() {
        let t = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(Point3D::new(5.0, 3.0, 4.0), &t * &p);
    }

    #[test]
    fn a_shearing_information_moves_x_in_propotion_to_z() {
        let t = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(Point3D::new(6.0, 3.0, 4.0), &t * &p);
    }

    #[test]
    fn a_shearing_information_moves_y_in_propotion_to_x() {
        let t = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(Point3D::new(2.0, 5.0, 4.0), &t * &p);
    }

    #[test]
    fn a_shearing_information_moves_y_in_propotion_to_z() {
        let t = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(Point3D::new(2.0, 7.0, 4.0), &t * &p);
    }

    #[test]
    fn a_shearing_information_moves_z_in_propotion_to_x() {
        let t = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(Point3D::new(2.0, 3.0, 6.0), &t * &p);
    }

    #[test]
    fn a_shearing_information_moves_z_in_propotion_to_y() {
        let t = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(Point3D::new(2.0, 3.0, 7.0), &t * &p);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Point3D::new(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(std::f32::consts::FRAC_PI_2);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translation(10.0, 5.0, 7.0);

        let p2 = &a * &p;
        assert_eq!(Point3D::new(1.0, -1.0, 0.0), p2);

        let p3 = &b * &p2;
        assert_eq!(Point3D::new(5.0, -5.0, 0.0), p3);

        let p4 = &c * &p3;
        assert_eq!(Point3D::new(15.0, 0.0, 7.0), p4);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Point3D::new(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(std::f32::consts::FRAC_PI_2);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translation(10.0, 5.0, 7.0);
        let t = &c * &(&b * &a);

        assert_eq!(Point3D::new(15.0, 0.0, 7.0), &t * &p);
    }
}
