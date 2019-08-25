use super::approx_eq;
use super::vector3d::Vector3D;

use std::ops::{Add, Div, Mul, Neg, Sub};

/// 3 次元空間内の 1 点 (x, y, z) を示す。
#[derive(Debug, Clone)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    /// 新しい Point3D を作成する
    ///
    /// # Argumets
    /// * `x` - x
    /// * `y` - y
    /// * `z` - z
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3D { x, y, z }
    }
}

impl PartialEq for Point3D {
    /// 2 つの Point3D が等しいかをテストする。
    /// float 同士の比較なので、ある程度の誤差を許容する。
    ///
    /// # Argumets
    ///
    /// * `other` - 比較対象となる Point3D
    fn eq(&self, other: &Point3D) -> bool {
        approx_eq(self.x, other.x)
            && approx_eq(self.y, other.y)
            && approx_eq(self.z, other.z)
    }
}

impl Add<&Vector3D> for &Point3D {
    type Output = Point3D;

    /// self から v だけ移動した新しい Point3D を得る
    ///
    /// Argumets
    ///
    /// * `v` - 移動分
    fn add(self, v: &Vector3D) -> Self::Output {
        Point3D::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Div<f32> for &Point3D {
    type Output = Point3D;

    /// self の各成分を 1/s 倍した Point3D を得る
    /// 0 除算のチェックは行わない
    ///
    /// Argumets
    ///
    /// * `s` - スケール
    fn div(self, s: f32) -> Self::Output {
        Point3D::new(self.x / s, self.y / s, self.z / s)
    }
}

impl Mul<f32> for &Point3D {
    type Output = Point3D;

    /// self の各成分を s 倍した Point3D を得る
    ///
    /// Argumets
    ///
    /// * `s` - スケール
    fn mul(self, s: f32) -> Self::Output {
        Point3D::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Neg for Point3D {
    type Output = Point3D;

    /// self の各成分の符号を逆転した Point3D を得る
    fn neg(self) -> Self::Output {
        Point3D::new(-self.x, -self.y, -self.z)
    }
}

impl Sub<&Point3D> for &Point3D {
    type Output = Vector3D;

    /// p から self へのVector3D を得る
    ///
    /// Argumets
    ///
    /// * `p` - 始点
    fn sub(self, p: &Point3D) -> Self::Output {
        Vector3D::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

impl Sub<&Vector3D> for &Point3D {
    type Output = Point3D;

    fn sub(self, v: &Vector3D) -> Self::Output {
        Point3D::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_creation() {
        let point = Point3D::new(4.3, -4.2, 3.1);
        assert_eq!(4.3, point.x);
        assert_eq!(-4.2, point.y);
        assert_eq!(3.1, point.z);
    }

    #[test]
    fn adding_a_vector() {
        let a1 = Point3D::new(3.0, -2.0, 5.0);
        let a2 = Vector3D::new(-2.0, 3.0, 1.0);

        assert_eq!(Point3D::new(1.0, 1.0, 6.0), &a1 + &a2);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Point3D::new(3.0, 2.0, 1.0);
        let p2 = Point3D::new(5.0, 6.0, 7.0);

        assert_eq!(Vector3D::new(-2.0, -4.0, -6.0), &p1 - &p2);
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Point3D::new(3.0, 2.0, 1.0);
        let v = Vector3D::new(5.0, 6.0, 7.0);

        assert_eq!(Point3D::new(-2.0, -4.0, -6.0), &p - &v);
    }

    #[test]
    fn negating_a_point() {
        let p = Point3D::new(1.0, -2.0, 3.0);

        assert_eq!(Point3D::new(-1.0, 2.0, -3.0), -p);
    }

    #[test]
    fn multiplying_a_point_by_a_scalar() {
        let p = Point3D::new(1.0, -2.0, 3.0);

        assert_eq!(Point3D::new(3.5, -7.0, 10.5), &p * 3.5);
    }

    #[test]
    fn multiplying_a_point_by_a_fraction() {
        let p = Point3D::new(1.0, -2.0, 3.0);

        assert_eq!(Point3D::new(0.5, -1.0, 1.5), &p * 0.5);
    }

    #[test]
    fn dividing_a_point_by_a_scalar() {
        let v = Point3D::new(1.0, -2.0, 3.0);

        assert_eq!(Point3D::new(0.5, -1.0, 1.5), &v / 2.0);
    }
}
