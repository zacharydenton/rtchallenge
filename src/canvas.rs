use crate::color::*;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl Canvas {
    /// Constructs a Canvas of the given width and height.
    ///
    /// Pixel data is stored as interleaved 8 bit RGB.
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            data: vec![0; 3 * width * height],
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        let i = 3 * (self.width * y + x);
        let r = self.data[i + 0] as f64 / 255.0;
        let g = self.data[i + 1] as f64 / 255.0;
        let b = self.data[i + 2] as f64 / 255.0;

        Color { r, g, b }
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let r = (color.r.max(0.0).min(1.0) * 255.0).round() as u8;
        let g = (color.g.max(0.0).min(1.0) * 255.0).round() as u8;
        let b = (color.b.max(0.0).min(1.0) * 255.0).round() as u8;
        let i = 3 * (self.width * y + x);

        self.data[i + 0] = r;
        self.data[i + 1] = g;
        self.data[i + 2] = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for component in c.data {
            assert_eq!(component, 0);
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.set_color(2, 3, red);
        assert_eq!(c.get_color(2, 3), red);
    }
}
