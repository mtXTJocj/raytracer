use super::approx_eq;

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
}
