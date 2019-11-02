use super::approx_eq;
use super::point3d::Point3D;
use std::ops::{Add, Div, DivAssign, Mul, Neg, Sub};

/// 3 次元空間内のベクトル (x, y, z) を示す。
#[derive(Debug, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    /// 新しい Vector3D を作成する
    ///
    /// # Argumets
    /// * `x` - x
    /// * `y` - y
    /// * `z` - z
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3D { x, y, z }
    }

    /// self のノルムを計算する
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// self を in-place に正規化する
    pub fn normalize(&mut self) -> &Self {
        let m = self.magnitude();
        *self /= m;

        self
    }

    /// self と v の内積を計算する
    ///
    /// # Argumets
    /// * `v` - Vector3D
    pub fn dot(&self, v: &Vector3D) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// self と v の外積を計算する
    ///
    /// # Argumets
    /// * `v` - Vector3D
    pub fn cross(&self, v: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

impl PartialEq for Vector3D {
    /// 2 つの Vector3D が等しいかをテストする。
    /// float 同士の比較なので、ある程度の誤差を許容する。
    ///
    /// # Argumets
    ///
    /// * `other` - 比較対象となる Vector3D
    fn eq(&self, other: &Vector3D) -> bool {
        approx_eq(self.x, other.x)
            && approx_eq(self.y, other.y)
            && approx_eq(self.z, other.z)
    }
}

impl Add<&Point3D> for &Vector3D {
    type Output = Point3D;

    /// p を self だけ移動した新しい Point3D を得る
    ///
    /// Argumets
    ///
    /// * `p` - 移動する Point3D
    fn add(self, p: &Point3D) -> Self::Output {
        Point3D::new(self.x + p.x, self.y + p.y, self.z + p.z)
    }
}

impl Add<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    /// 2 つの Vector3D の和を計算する。
    ///
    /// # Argumets
    ///
    /// * `v` - self に足す Vector3D
    fn add(self, v: &Vector3D) -> Self::Output {
        Vector3D::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub<&Vector3D> for &Vector3D {
    type Output = Vector3D;

    /// 2 つの Vector3D の差を計算する。
    ///
    /// # Argumets
    ///
    /// * `v` - self から引く Vector3D
    fn sub(self, v: &Vector3D) -> Self::Output {
        Vector3D::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Neg for &Vector3D {
    type Output = Vector3D;

    /// self と反対のベクトルを得る
    fn neg(self) -> Self::Output {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for &Vector3D {
    type Output = Vector3D;

    /// self を s 倍する
    ///
    /// # Argumets
    ///
    /// * `s` - self にかけるスカラー値
    fn mul(self, s: f32) -> Self::Output {
        Vector3D::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Div<f32> for &Vector3D {
    type Output = Vector3D;

    /// self を 1/s 倍する。
    ///
    /// # Argumets
    ///
    /// * `s` - self を割るスカラー値
    fn div(self, s: f32) -> Self::Output {
        Vector3D::new(self.x / s, self.y / s, self.z / s)
    }
}

impl DivAssign<f32> for Vector3D {
    /// self を 1/s 倍する。
    ///
    /// # Argumets
    ///
    /// * `s` - self を割るスカラー値
    fn div_assign(&mut self, s: f32) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_creation() {
        let vector = Vector3D::new(4.3, -4.2, 3.1);
        assert_eq!(4.3, vector.x);
        assert_eq!(-4.2, vector.y);
        assert_eq!(3.1, vector.z);
    }

    #[test]
    fn adding_a_point() {
        let a1 = Vector3D::new(3.0, -2.0, 5.0);
        let a2 = Point3D::new(-2.0, 3.0, 1.0);

        assert_eq!(Point3D::new(1.0, 1.0, 6.0), &a1 + &a2);
    }

    #[test]
    fn adding_two_vectors() {
        let a1 = Vector3D::new(3.0, -2.0, 5.0);
        let a2 = Vector3D::new(-2.0, 3.0, 1.0);

        assert_eq!(Vector3D::new(1.0, 1.0, 6.0), &a1 + &a2);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector3D::new(3.0, 2.0, 1.0);
        let v2 = Vector3D::new(5.0, 6.0, 7.0);

        assert_eq!(Vector3D::new(-2.0, -4.0, -6.0), &v1 - &v2);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let v1 = Vector3D::new(0.0, 0.0, 0.0);
        let v2 = Vector3D::new(1.0, -2.0, 3.0);

        assert_eq!(Vector3D::new(-1.0, 2.0, -3.0), &v1 - &v2);
    }

    #[test]
    fn negating_a_vector() {
        let v = Vector3D::new(1.0, -2.0, 3.0);

        assert_eq!(Vector3D::new(-1.0, 2.0, -3.0), -&v);
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let v = Vector3D::new(1.0, -2.0, 3.0);

        assert_eq!(Vector3D::new(3.5, -7.0, 10.5), &v * 3.5);
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let v = Vector3D::new(1.0, -2.0, 3.0);

        assert_eq!(Vector3D::new(0.5, -1.0, 1.5), &v * 0.5);
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let v = Vector3D::new(1.0, -2.0, 3.0);

        assert_eq!(Vector3D::new(0.5, -1.0, 1.5), &v / 2.0);
    }

    #[test]
    fn dividing_a_vector_by_a_scalar_and_assign() {
        let mut v = Vector3D::new(1.0, -2.0, 3.0);

        v /= 2.0;
        assert_eq!(Vector3D::new(0.5, -1.0, 1.5), v);
    }

    #[test]
    fn computing_the_magnitude_of_vector() {
        assert_eq!(1.0, Vector3D::new(1.0, 0.0, 0.0).magnitude());
        assert_eq!(1.0, Vector3D::new(0.0, 1.0, 0.0).magnitude());
        assert_eq!(1.0, Vector3D::new(0.0, 0.0, 1.0).magnitude());

        assert_eq!(14f32.sqrt(), Vector3D::new(1.0, 2.0, 3.0).magnitude());
        assert_eq!(14f32.sqrt(), Vector3D::new(-1.0, -2.0, -3.0).magnitude());
    }

    #[test]
    fn normalizing_a_vector() {
        let mut v = Vector3D::new(4.0, 0.0, 0.0);
        assert_eq!(Vector3D::new(1.0, 0.0, 0.0), *v.normalize());

        let mut v = Vector3D::new(1.0, 2.0, 3.0);
        let m = 14f32.sqrt();
        assert_eq!(Vector3D::new(1.0 / m, 2.0 / m, 3.0 / m), *v.normalize());
        assert!(approx_eq(1.0, v.magnitude()));
    }

    #[test]
    fn the_dot_product_of_two_vectors() {
        let a = Vector3D::new(1.0, 2.0, 3.0);
        let b = Vector3D::new(2.0, 3.0, 4.0);

        assert_eq!(20.0, a.dot(&b));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Vector3D::new(1.0, 2.0, 3.0);
        let b = Vector3D::new(2.0, 3.0, 4.0);

        assert_eq!(Vector3D::new(-1.0, 2.0, -1.0), a.cross(&b));
        assert_eq!(Vector3D::new(1.0, -2.0, 1.0), b.cross(&a));
    }
}
