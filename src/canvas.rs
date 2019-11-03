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
}
