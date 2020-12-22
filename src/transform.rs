use super::{
    matrix4x4::Matrix4x4, point3d::Point3D, ray::Ray, vector3d::Vector3D, FLOAT,
};
use std::{cmp::PartialEq, ops::Mul};

/// 座標変換を表す。
#[derive(Debug)]
pub struct Transform {
    mat: Matrix4x4,
    inv: Matrix4x4,
}

impl Transform {
    /// 恒等変換を作成する
    pub fn identity() -> Self {
        Transform {
            mat: Matrix4x4::identity(),
            inv: Matrix4x4::identity(),
        }
    }

    /// 平行移動の変換を作成する
    ///
    /// # Argumets
    /// * `x` - x 方向の移動量
    /// * `y` - y 方向の移動量
    /// * `z` - z 方向の移動量
    pub fn translation(x: FLOAT, y: FLOAT, z: FLOAT) -> Self {
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

    /// 拡大/縮小の変換を作成する
    ///
    /// # Argumets
    /// * `x` - x 方向のスケール
    /// * `y` - y 方向のスケール
    /// * `z` - z 方向のスケール
    pub fn scaling(x: FLOAT, y: FLOAT, z: FLOAT) -> Self {
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

    /// x 軸まわりの回転を作成する
    ///
    /// # Argumets
    /// * `a` - 回転角(rad)
    pub fn rotation_x(a: FLOAT) -> Self {
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

    /// y 軸まわりの回転を作成する
    ///
    /// # Argumets
    /// * `a` - 回転角(rad)
    pub fn rotation_y(a: FLOAT) -> Self {
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

    /// z 軸まわりの回転を作成する
    ///
    /// # Argumets
    /// * `a` - 回転角(rad)
    pub fn rotation_z(a: FLOAT) -> Self {
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

    /// 剪断用の変換を作成する
    ///
    /// # Argumets
    /// * `xy` - y の変化量に対する x の変化量
    /// * `xz` - z の変化量に対する x の変化量
    /// * `yx` - x の変化量に対する y の変化量
    /// * `yz` - z の変化量に対する y の変化量
    /// * `zx` - x の変化量に対する z の変化量
    /// * `zy` - y の変化量に対する z の変化量
    pub fn shearing(
        xy: FLOAT,
        xz: FLOAT,
        yx: FLOAT,
        yz: FLOAT,
        zx: FLOAT,
        zy: FLOAT,
    ) -> Self {
        let mat = Matrix4x4::new([
            1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0,
            0.0, 1.0,
        ]);
        let inv = mat.inverse();
        Transform { mat, inv }
    }

    pub fn view_transform(from: &Point3D, to: &Point3D, up: &Vector3D) -> Self {
        let mut forward = to - from;
        forward.normalize();
        let mut normalized_up = up.clone();
        normalized_up.normalize();
        let left = forward.cross(&normalized_up);
        let true_up = left.cross(&forward);

        let orientation = Matrix4x4::new([
            left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0,
            -forward.x, -forward.y, -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]);
        let translation = Matrix4x4::new([
            1.0, 0.0, 0.0, -from.x, 0.0, 1.0, 0.0, -from.y, 0.0, 0.0, 1.0,
            -from.z, 0.0, 0.0, 0.0, 1.0,
        ]);
        let mat = &orientation * &translation;
        let inv = mat.inverse();

        Transform { mat, inv }
    }

    /// 逆変換の行列を取得する
    pub fn inv(&self) -> &Matrix4x4 {
        &self.inv
    }

    pub fn apply_to_normal(&self, n: &Vector3D) -> Vector3D {
        let m = &self.inv;

        let x = m.at(0, 0) * n.x + m.at(1, 0) * n.y + m.at(2, 0) * n.z;
        let y = m.at(0, 1) * n.x + m.at(1, 1) * n.y + m.at(2, 1) * n.z;
        let z = m.at(0, 2) * n.x + m.at(1, 2) * n.y + m.at(2, 2) * n.z;

        let mut n = Vector3D::new(x, y, z);
        n.normalize();
        n
    }
}

impl PartialEq<Transform> for Transform {
    fn eq(&self, other: &Transform) -> bool {
        self.mat == other.mat
    }
}

impl Mul<&Point3D> for &Transform {
    type Output = Point3D;

    /// Point3D に対して変換を適用する
    ///
    /// # Arguments
    /// * `x` 適用対象となる Point3D
    fn mul(self, x: &Point3D) -> Self::Output {
        &self.mat * x
    }
}

impl Mul<&Vector3D> for &Transform {
    type Output = Vector3D;

    /// Vector3D に対して変換を適用する
    ///
    /// # Arguments
    /// * `x` 適用対象となる Vector3D
    fn mul(self, x: &Vector3D) -> Self::Output {
        &self.mat * x
    }
}

impl Mul<&Ray> for &Transform {
    type Output = Ray;

    /// Ray に対して変換を適用する
    ///
    /// # Arguments
    /// * `r` 適用対象となる Ray
    fn mul(self, r: &Ray) -> Self::Output {
        &self.mat * r
    }
}

impl Mul<&Transform> for &Transform {
    type Output = Transform;

    /// 2 つの Transform を合成する
    ///
    /// # Arguments
    /// * `t` 右からかける Transform
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
        let half_quarter =
            Transform::rotation_x(std::f32::consts::FRAC_PI_4 as FLOAT);
        let full_quarter =
            Transform::rotation_x(std::f32::consts::FRAC_PI_2 as FLOAT);

        assert_eq!(
            Point3D::new(
                0.0,
                2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0
            ),
            &half_quarter * &p
        );
        assert_eq!(Point3D::new(0.0, 0.0, 1.0), &full_quarter * &p);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter =
            Transform::rotation_x(std::f32::consts::FRAC_PI_4 as FLOAT);

        assert_eq!(
            Point3D::new(
                0.0,
                2f32.sqrt() as FLOAT / 2.0,
                -2f32.sqrt() as FLOAT / 2.0
            ),
            half_quarter.inv() * &p
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Point3D::new(0.0, 0.0, 1.0);
        let half_quarter =
            Transform::rotation_y(std::f32::consts::FRAC_PI_4 as FLOAT);
        let full_quarter =
            Transform::rotation_y(std::f32::consts::FRAC_PI_2 as FLOAT);

        assert_eq!(
            Point3D::new(
                2f32.sqrt() as FLOAT / 2.0,
                0.0,
                2f32.sqrt() as FLOAT / 2.0
            ),
            &half_quarter * &p
        );
        assert_eq!(Point3D::new(1.0, 0.0, 0.0), &full_quarter * &p);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let half_quarter =
            Transform::rotation_z(std::f32::consts::FRAC_PI_4 as FLOAT);
        let full_quarter =
            Transform::rotation_z(std::f32::consts::FRAC_PI_2 as FLOAT);

        assert_eq!(
            Point3D::new(
                -2f32.sqrt() as FLOAT / 2.0,
                2f32.sqrt() as FLOAT / 2.0,
                0.0
            ),
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
        let a = Transform::rotation_x(std::f32::consts::FRAC_PI_2 as FLOAT);
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
        let a = Transform::rotation_x(std::f32::consts::FRAC_PI_2 as FLOAT);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translation(10.0, 5.0, 7.0);
        let t = &c * &(&b * &a);

        assert_eq!(Point3D::new(15.0, 0.0, 7.0), &t * &p);
    }

    #[test]
    fn chained_inverse_transformations_must_be_applied_in_reverse_order() {
        let p = Point3D::new(15.0, 0.0, 7.0);
        let a = Transform::rotation_x(std::f32::consts::FRAC_PI_2 as FLOAT);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translation(10.0, 5.0, 7.0);
        let t = &c * &(&b * &a);

        assert_eq!(Point3D::new(1.0, 0.0, 1.0), t.inv() * &p);
    }

    #[test]
    fn translating_a_ray() {
        let r =
            Ray::new(Point3D::new(1.0, 2.0, 3.0), Vector3D::new(0.0, 1.0, 0.0));
        let m = Transform::translation(3.0, 4.0, 5.0);

        let r2 = &m * &r;

        assert_eq!(Point3D::new(4.0, 6.0, 8.0), *r2.origin());
        assert_eq!(Vector3D::new(0.0, 1.0, 0.0), *r2.direction());
    }

    #[test]
    fn scaling_a_ray() {
        let r =
            Ray::new(Point3D::new(1.0, 2.0, 3.0), Vector3D::new(0.0, 1.0, 0.0));
        let m = Transform::scaling(2.0, 3.0, 4.0);

        let r2 = &m * &r;

        assert_eq!(Point3D::new(2.0, 6.0, 12.0), *r2.origin());
        assert_eq!(Vector3D::new(0.0, 3.0, 0.0), *r2.direction());
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Point3D::new(0.0, 0.0, 0.0);
        let to = Point3D::new(0.0, 0.0, -1.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);

        let t = Transform::view_transform(&from, &to, &up);
        assert_eq!(Transform::identity(), t);
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z() {
        let from = Point3D::new(0.0, 0.0, 0.0);
        let to = Point3D::new(0.0, 0.0, 1.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);

        let t = Transform::view_transform(&from, &to, &up);
        assert_eq!(Transform::scaling(-1.0, 1.0, -1.0), t);
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point3D::new(0.0, 0.0, 8.0);
        let to = Point3D::new(0.0, 0.0, 0.0);
        let up = Vector3D::new(0.0, 1.0, 0.0);

        let t = Transform::view_transform(&from, &to, &up);
        assert_eq!(Transform::translation(0.0, 0.0, -8.0), t);
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Point3D::new(1.0, 3.0, 2.0);
        let to = Point3D::new(4.0, -2.0, 8.0);
        let up = Vector3D::new(1.0, 1.0, 0.0);

        let t = Transform::view_transform(&from, &to, &up);

        let mat = Matrix4x4::new([
            -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122,
            -2.82843, -0.35857, 0.59761, -0.71714, 0.00000, 0.00000, 0.00000,
            0.00000, 1.0,
        ]);
        let inv = mat.inverse();
        assert_eq!(Transform { mat, inv }, t);
    }
}
