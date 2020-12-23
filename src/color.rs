use std::ops::{Add, Mul, Sub};

use super::{approx_eq, FLOAT};

/// 色を RGB で表す
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: FLOAT,
    pub green: FLOAT,
    pub blue: FLOAT,
}

impl Color {
    /// 黒
    pub const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    /// 白
    pub const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    /// 赤
    pub const RED: Color = Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    };

    /// 新しい Color を作成する
    ///
    /// # Argumets
    /// * `red` - red
    /// * `green` - green
    /// * `blue` - blue
    pub fn new(red: FLOAT, green: FLOAT, blue: FLOAT) -> Self {
        Color { red, green, blue }
    }
}

impl PartialEq for Color {
    /// 2 つの Color が等しいかをテストする。
    /// float 同士の比較なので、ある程度の誤差を許容する。
    ///
    /// # Argumets
    ///
    /// * `other` - 比較対象となる Color
    fn eq(&self, other: &Color) -> bool {
        approx_eq(self.red, other.red)
            && approx_eq(self.green, other.green)
            && approx_eq(self.blue, other.blue)
    }
}

impl Add<&Color> for &Color {
    type Output = Color;

    /// 2 つの Color を加算する
    ///
    /// Argumets
    ///
    /// * `other` - 加算する Color
    fn add(self, other: &Color) -> Self::Output {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Sub<&Color> for &Color {
    type Output = Color;

    /// 2 つの Color を減算する
    ///
    /// Argumets
    ///
    /// * `other` - 減算する Color
    fn sub(self, other: &Color) -> Self::Output {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Mul<FLOAT> for &Color {
    type Output = Color;

    /// Color の各要素を other 倍する
    ///
    /// Argumets
    ///
    /// * `other` - 乗算する FLOAT
    fn mul(self, other: FLOAT) -> Self::Output {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl Mul<&Color> for FLOAT {
    type Output = Color;

    fn mul(self, other: &Color) -> Self::Output {
        Color::new(self * other.red, self * other.green, self * other.blue)
    }
}

impl Mul<&Color> for &Color {
    type Output = Color;

    /// 2 つの Color を乗算する
    ///
    /// Argumets
    ///
    /// * `other` - 乗算する Color
    fn mul(self, other: &Color) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_creation() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, c.red);
        assert_eq!(0.4, c.green);
        assert_eq!(1.7, c.blue);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(Color::new(1.6, 0.7, 1.0), &c1 + &c2);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(Color::new(0.2, 0.5, 0.5), &c1 - &c2);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(Color::new(0.9, 0.2, 0.04), &c1 * &c2);
    }
}
