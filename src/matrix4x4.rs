use std::{cmp::PartialEq, ops::Mul};

use super::{approx_eq, point3d::Point3D, vector3d::Vector3D};

#[derive(Debug)]
pub struct Matrix4x4 {
    m: [f32; 16],
}

impl Matrix4x4 {
    pub fn new(m: [f32; 16]) -> Self {
        Matrix4x4 { m }
    }

    pub fn identity() -> Self {
        Matrix4x4 {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn at(&self, row: usize, column: usize) -> f32 {
        debug_assert!(row < 4 && column < 4);

        self.m[row * 4 + column]
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Matrix4x4) -> bool {
        self.m
            .iter()
            .zip(other.m.iter())
            .fold(true, |result, (a, b)| result && approx_eq(*a, *b))
    }
}

impl Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, mat: &Matrix4x4) -> Self::Output {
        let mut m = [0.0; 16];

        for r in 0..4 {
            for c in 0..4 {
                m[r * 4 + c] = self.at(r, 0) * mat.at(0, c)
                    + self.at(r, 1) * mat.at(1, c)
                    + self.at(r, 2) * mat.at(2, c)
                    + self.at(r, 3) * mat.at(3, c);
            }
        }

        Matrix4x4::new(m)
    }
}

impl Mul<&Point3D> for &Matrix4x4 {
    type Output = Point3D;

    fn mul(self, p: &Point3D) -> Self::Output {
        let x = self.at(0, 0) * p.x
            + self.at(0, 1) * p.y
            + self.at(0, 2) * p.z
            + self.at(0, 3);
        let y = self.at(1, 0) * p.x
            + self.at(1, 1) * p.y
            + self.at(1, 2) * p.z
            + self.at(1, 3);
        let z = self.at(2, 0) * p.x
            + self.at(2, 1) * p.y
            + self.at(2, 2) * p.z
            + self.at(2, 3);

        Point3D::new(x, y, z)
    }
}

impl Mul<&Vector3D> for &Matrix4x4 {
    type Output = Vector3D;

    fn mul(self, p: &Vector3D) -> Self::Output {
        let x = self.at(0, 0) * p.x + self.at(0, 1) * p.y + self.at(0, 2) * p.z;
        let y = self.at(1, 0) * p.x + self.at(1, 1) * p.y + self.at(1, 2) * p.z;
        let z = self.at(2, 0) * p.x + self.at(2, 1) * p.y + self.at(2, 2) * p.z;

        Vector3D::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(1.0, m.at(0, 0));
        assert_eq!(4.0, m.at(0, 3));
        assert_eq!(5.5, m.at(1, 0));
        assert_eq!(7.5, m.at(1, 2));
        assert_eq!(11.0, m.at(2, 2));
        assert_eq!(13.5, m.at(3, 0));
        assert_eq!(15.5, m.at(3, 2));
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let m = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0,
        ];
        let m0 = Matrix4x4::new(m);
        let m1 = Matrix4x4::new(m);

        assert_eq!(m0, m1);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let m0 = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0,
        ]);
        let m1 = Matrix4x4::new([
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0,
            3.0, 2.0, 1.0,
        ]);

        assert_ne!(m0, m1);
    }

    #[test]
    fn multiplying_two_matrices() {
        let mat_a = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0,
        ]);
        let mat_b = Matrix4x4::new([
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0,
            2.0, 7.0, 8.0,
        ]);

        assert_eq!(
            Matrix4x4::new([
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0,
                110.0, 102.0, 16.0, 26.0, 46.0, 42.0
            ]),
            &mat_a * &mat_b
        );
    }

    #[test]
    fn a_matrix_multiplied_by_a_point() {
        let mat_a = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]);
        let p = Point3D::new(1.0, 2.0, 3.0);

        assert_eq!(Point3D::new(18.0, 24.0, 33.0), &mat_a * &p);
    }

    #[test]
    fn a_matrix_multiplied_by_a_vector() {
        let mat_a = Matrix4x4::new([
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]);
        let v = Vector3D::new(1.0, 2.0, 3.0);

        assert_eq!(Vector3D::new(14.0, 22.0, 32.0), &mat_a * &v);
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let m = [
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0,
            8.0, 16.0, 32.0,
        ];
        let mat_a = Matrix4x4::new(m);

        assert_eq!(mat_a, &mat_a * &Matrix4x4::identity());
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_point() {
        let p = Point3D::new(1.0, 2.0, 3.0);

        assert_eq!(p, &Matrix4x4::identity() * &p);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_vector() {
        let v = Vector3D::new(1.0, 2.0, 3.0);

        assert_eq!(v, &Matrix4x4::identity() * &v);
    }
}
