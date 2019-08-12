use super::approx_eq;

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
}
