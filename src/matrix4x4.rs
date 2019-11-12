use std::cmp::PartialEq;

use super::approx_eq;

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

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Matrix4x4) -> bool {
        self.m
            .iter()
            .zip(other.m.iter())
            .fold(true, |result, (a, b)| result && approx_eq(*a, *b))
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
}
