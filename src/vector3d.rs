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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let vector = Vector3D::new(4.3, -4.2, 3.1);
        assert_eq!(4.3, vector.x);
        assert_eq!(-4.2, vector.y);
        assert_eq!(3.1, vector.z);
    }
}
