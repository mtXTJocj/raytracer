use super::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    colors: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            colors: vec![Color::BLACK; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn color_at(&self, x: usize, y: usize) -> &Color {
        assert!(x < self.width);
        assert!(y < self.height);

        &self.colors[self.width * y + x]
    }

    pub fn color_at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        assert!(x < self.width);
        assert!(y < self.height);

        &mut self.colors[self.width * y + x]
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
}
