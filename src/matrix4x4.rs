use std::{cmp::PartialEq, ops::Mul};

use super::{approx_eq, point3d::Point3D, ray::Ray, vector3d::Vector3D, FLOAT};

/// 4x4 行列を表す。
#[derive(Debug)]
pub struct Matrix4x4 {
    m: [FLOAT; 16],
}

impl Matrix4x4 {
    /// 新しい Vector3D を作成する
    ///
    /// # Argumets
    /// * `m` - row-major で各要素を格納した配列
    pub fn new(m: [FLOAT; 16]) -> Self {
        Matrix4x4 { m }
    }

    /// 単位行列を作成する
    pub fn identity() -> Self {
        Matrix4x4 {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// self の転置行列を作成する
    pub fn transpose(&self) -> Self {
        let mut m = [0.0; 16];
        for r in 0..4 {
            for c in 0..4 {
                m[c * 4 + r] = self.m[r * 4 + c]
            }
        }
        Matrix4x4 { m }
    }

    /// 行列の要素を取得する。
    ///
    /// # Argumets
    /// * `row` - 行 [0, 3]
    /// * `column` - 列 [0, 3]
    pub fn at(&self, row: usize, column: usize) -> FLOAT {
        debug_assert!(row < 4 && column < 4);

        self.m[row * 4 + column]
    }

    fn submatrix(&self, row: usize, column: usize) -> Matrix3x3 {
        let mut m = [0.0; 9];

        let mut i = 0;
        for r in 0..4 {
            if r != row {
                for c in 0..4 {
                    if c != column {
                        m[i] = self.at(r, c);
                        i += 1;
                    }
                }
            }
        }
        Matrix3x3::new(m)
    }

    fn minor(&self, row: usize, column: usize) -> FLOAT {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> FLOAT {
        let m = self.minor(row, column);
        if (row + column) & 0x1 == 0 {
            m
        } else {
            -m
        }
    }

    fn determinant(&self) -> FLOAT {
        (0..4).map(|i| self.m[i] * self.cofactor(0, i)).sum()
    }

    /// self の逆行列を作成する。
    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        if det == 0.0 {
            panic!();
        }

        let inv_det = 1.0 / det;
        let mut m = [0.0; 16];
        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                // transpose するため、col と row を逆にしている
                m[col * 4 + row] = c * inv_det
            }
        }

        Matrix4x4::new(m)
    }
}

impl PartialEq for Matrix4x4 {
    /// 2 つの Matrix4x4 が等しいかをテストする。
    /// float 同士の比較なので、ある程度の誤差を許容する。
    ///
    /// # Argumets
    ///
    /// * `other` - 比較対象となる Matrix4x4
    fn eq(&self, other: &Matrix4x4) -> bool {
        self.m
            .iter()
            .zip(other.m.iter())
            .fold(true, |result, (a, b)| result && approx_eq(*a, *b))
    }
}

impl Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;

    /// Matrix4x4 同士の積
    ///
    /// # Argumets
    ///
    /// * `other` - 乗算対象となる Matrix4x4
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

    /// self * p を計算する
    ///
    /// # Argumets
    ///
    /// * `p` - 対象となる Point3D
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

    /// self * p を計算する
    ///
    /// # Argumets
    ///
    /// * `p` - 対象となる Point3D
    fn mul(self, p: &Vector3D) -> Self::Output {
        let x = self.at(0, 0) * p.x + self.at(0, 1) * p.y + self.at(0, 2) * p.z;
        let y = self.at(1, 0) * p.x + self.at(1, 1) * p.y + self.at(1, 2) * p.z;
        let z = self.at(2, 0) * p.x + self.at(2, 1) * p.y + self.at(2, 2) * p.z;

        Vector3D::new(x, y, z)
    }
}

impl Mul<&Ray> for &Matrix4x4 {
    type Output = Ray;

    /// Ray に対して変換行列を適用する。
    ///
    /// # Argumets
    ///
    /// * `r` - 対象となる Ray
    fn mul(self, r: &Ray) -> Self::Output {
        let o = self * r.origin();
        let d = self * r.direction();

        Ray::new(o, d)
    }
}

/* ------------------------------------------------------------------------- */

#[derive(Debug)]
struct Matrix3x3 {
    m: [FLOAT; 9],
}

impl Matrix3x3 {
    fn new(m: [FLOAT; 9]) -> Self {
        Matrix3x3 { m }
    }

    fn at(&self, row: usize, column: usize) -> FLOAT {
        self.m[row * 3 + column]
    }

    fn submatrix(&self, row: usize, column: usize) -> Matrix2x2 {
        let mut m = [0.0; 4];

        let mut i = 0;
        for r in 0..3 {
            if r != row {
                for c in 0..3 {
                    if c != column {
                        m[i] = self.at(r, c);
                        i += 1;
                    }
                }
            }
        }
        Matrix2x2::new(m)
    }

    fn minor(&self, row: usize, column: usize) -> FLOAT {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> FLOAT {
        let m = self.minor(row, column);
        if (row + column) & 0x1 == 0 {
            m
        } else {
            -m
        }
    }

    fn determinant(&self) -> FLOAT {
        (0..3).map(|i| self.m[i] * self.cofactor(0, i)).sum()
    }
}

/* ------------------------------------------------------------------------- */

#[derive(Debug)]
struct Matrix2x2 {
    m: [FLOAT; 4],
}

impl Matrix2x2 {
    fn new(m: [FLOAT; 4]) -> Self {
        Matrix2x2 { m }
    }

    fn determinant(&self) -> FLOAT {
        self.m[0] * self.m[3] - self.m[1] * self.m[2]
    }
}

/* ------------------------------------------------------------------------- */

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

    #[test]
    fn transposing_a_matrix() {
        let mat = Matrix4x4::new([
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0,
            0.0, 5.0, 8.0,
        ]);
        let mat = mat.transpose();
        assert_eq!(
            Matrix4x4::new([
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0,
                0.0, 8.0, 3.0, 8.0
            ]),
            mat
        );
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let mat = Matrix4x4::identity();
        let mat = mat.transpose();
        assert_eq!(Matrix4x4::identity(), mat);
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let mat = Matrix2x2::new([1.0, 5.0, -3.0, 2.0]);
        assert_eq!(17.0, mat.determinant());
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let mat =
            Matrix3x3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        assert_eq!([-3.0, 2.0, 0.0, 6.0], mat.submatrix(0, 2).m);
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let mat = Matrix4x4::new([
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0,
        ]);
        assert_eq!(
            [-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0],
            mat.submatrix(2, 1).m
        );
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let mat =
            Matrix3x3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(25.0, mat.submatrix(1, 0).determinant());
        assert_eq!(25.0, mat.minor(1, 0));
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let mat =
            Matrix3x3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(-12.0, mat.minor(0, 0));
        assert_eq!(-12.0, mat.cofactor(0, 0));
        assert_eq!(25.0, mat.minor(1, 0));
        assert_eq!(-25.0, mat.cofactor(1, 0));
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let mat =
            Matrix3x3::new([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        assert_eq!(56.0, mat.cofactor(0, 0));
        assert_eq!(12.0, mat.cofactor(0, 1));
        assert_eq!(-46.0, mat.cofactor(0, 2));
        assert_eq!(-196.0, mat.determinant());
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let mat = Matrix4x4::new([
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0,
        ]);

        assert_eq!(690.0, mat.cofactor(0, 0));
        assert_eq!(447.0, mat.cofactor(0, 1));
        assert_eq!(210.0, mat.cofactor(0, 2));
        assert_eq!(51.0, mat.cofactor(0, 3));
        assert_eq!(-4071.0, mat.determinant());
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let mat = Matrix4x4::new([
            6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0,
            1.0, 7.0, -6.0,
        ]);
        assert_eq!(-2120.0, mat.determinant());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let mat = Matrix4x4::new([
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0,
        ]);
        assert_eq!(0.0, mat.determinant());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let mat = Matrix4x4::new([
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0,
        ]);
        assert_eq!(
            Matrix4x4::new([
                0.2180451,
                0.4511278,
                0.2406015,
                -0.0451127,
                -0.8082707,
                -1.456767,
                -0.4436090,
                0.520676,
                -0.07894736,
                -0.223684,
                -0.0526315,
                0.1973684,
                -0.5225563,
                -0.8139097,
                -0.3007518,
                0.3063909,
            ]),
            mat.inverse()
        );
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let mat = Matrix4x4::new([
            8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0,
            0.0, -9.0, -4.0,
        ]);
        assert_eq!(
            Matrix4x4::new([
                -0.1538461, -0.1538461, -0.282051, -0.5384615, -0.0769230,
                0.1230769, 0.02564102, 0.0307692, 0.3589743, 0.3589743,
                0.4358974, 0.923076, -0.692307, -0.692307, -0.769230,
                -1.923076
            ]),
            mat.inverse()
        )
    }

    #[test]
    fn calculating_the_inverse_of_third_matrix() {
        let mat = Matrix4x4::new([
            9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0,
        ]);
        assert_eq!(
            Matrix4x4::new([
                -0.04074,
                -0.077777,
                0.144444,
                -0.222222,
                -0.0777778,
                0.0333333,
                0.366666,
                -0.3333333,
                -0.02901234,
                -0.1462962,
                -0.10926,
                0.12963,
                0.1777777,
                0.066666,
                -0.266666,
                0.3333333
            ]),
            mat.inverse()
        );
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix4x4::new([
            3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0,
        ]);
        let b = Matrix4x4::new([
            8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0,
            -2.0, 0.0, 5.0,
        ]);

        let c = &a * &b;
        assert_eq!(a, &c * &b.inverse());
    }
}
