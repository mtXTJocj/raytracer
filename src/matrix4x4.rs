#[derive(Debug)]
pub struct Matrix4x4 {
    m: [f32; 16],
}

impl Matrix4x4 {
    pub fn new(m: [f32; 16]) -> Self {
        Matrix4x4 { m }
    }

    pub fn at(&self, row: usize, column: usize) -> f32 {
        debug_assert!(row < 4 && column < 4);

        self.m[row * 4 + column]
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
}
