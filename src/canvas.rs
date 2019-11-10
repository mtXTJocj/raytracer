use super::color::Color;
use std::io::{Result, Write};

/// 2 次元のイメージを表す。
/// 左上が原点
pub struct Canvas {
    /// 幅
    width: usize,
    /// 高さ
    height: usize,
    /// 色の配列
    colors: Vec<Color>,
}

impl Canvas {
    /// Canvas を作成する
    /// 作成時は黒で塗りつぶされている
    ///
    /// # Argumets
    /// * `width` - 幅
    /// * `height` - 高さ
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            colors: vec![Color::BLACK; width * height],
        }
    }

    /// Canvas の幅
    pub fn width(&self) -> usize {
        self.width
    }

    /// Canvas の高さ
    pub fn height(&self) -> usize {
        self.height
    }

    /// Canvas の (x, y) における色を取得する
    ///
    /// # Argumets
    /// * `x` - x
    /// * `y` - y
    pub fn color_at(&self, x: usize, y: usize) -> &Color {
        assert!(x < self.width);
        assert!(y < self.height);

        &self.colors[self.width * y + x]
    }

    /// Canvas の (x, y) における色を取得する
    ///
    /// # Argumets
    /// * `x` - x
    /// * `y` - y
    pub fn color_at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        assert!(x < self.width);
        assert!(y < self.height);

        &mut self.colors[self.width * y + x]
    }

    /// Canvas の内容を PPM 形式にして出力する。
    /// 出力に成功した場合、出力したバイト数を返す。
    ///
    /// # Argumets
    /// * `dst` - 出力先
    ///
    /// # Failures
    /// 出力に失敗
    pub fn to_ppm(&self, dst: &mut dyn Write) -> Result<usize> {
        let mut result = 0;
        result += dst.write(
            format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes(),
        )?;

        for i in 0..self.height {
            for j in 0..self.width {
                let c = self.color_at(j, i);
                let r = (c.red * 255.0).round().min(255.0).max(0.0) as u8;
                let g = (c.green * 255.0).round().min(255.0).max(0.0) as u8;
                let b = (c.blue * 255.0).round().min(255.0).max(0.0) as u8;

                result +=
                    dst.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let w = 10;
        let h = 20;
        let c = Canvas::new(w, h);

        assert_eq!(w, c.width());
        assert_eq!(h, c.height());
        for i in 0..h {
            for j in 0..w {
                assert_eq!(Color::BLACK, *c.color_at(j, i));
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        *c.color_at_mut(0, 0) = red;
        assert_eq!(red, *c.color_at(0, 0));
        *c.color_at_mut(2, 3) = red;
        assert_eq!(red, *c.color_at(2, 3));
        *c.color_at_mut(9, 19) = red;
        assert_eq!(red, *c.color_at(9, 19));
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let mut dst: Vec<u8> = Vec::new();

        let _result = c.to_ppm(&mut dst).unwrap();
        assert_eq!(
            r"P3
5 3
255
"
            .as_bytes(),
            &dst[..11]
        );
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let mut dst: Vec<u8> = Vec::new();

        dst.clear();
        *c.color_at_mut(0, 0) = Color::new(1.5, 0.0, 0.0);
        *c.color_at_mut(2, 1) = Color::new(0.0, 0.5, 0.0);
        *c.color_at_mut(4, 2) = Color::new(-0.5, 0.0, 1.0);
        let _result = c.to_ppm(&mut dst).unwrap();
        assert_eq!(
            r"P3
5 3
255
255 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 128 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 255
"
            .as_bytes(),
            &dst[..]
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let width = 10;
        let height = 2;
        let mut c = Canvas::new(width, height);

        for y in 0..height {
            for x in 0..width {
                *c.color_at_mut(x, y) = Color::new(1.0, 0.8, 0.6);
            }
        }

        let mut ppm = Vec::new();
        let _result = c.to_ppm(&mut ppm);

        assert_eq!(
            r"P3
10 2
255
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
"
            .as_bytes(),
            &ppm[..]
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let mut ppm = Vec::new();
        let c = Canvas::new(5, 3);
        let _result = c.to_ppm(&mut ppm);

        assert_eq!('\n', char::from(ppm[ppm.len() - 1]));
    }
}
