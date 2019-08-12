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
