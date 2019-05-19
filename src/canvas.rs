use crate::color::*;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

/// Constructs a Canvas of the given width and height.
///
/// Pixel data is stored as interleaved 8 bit RGB.
pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas {
        width,
        height,
        data: vec![0; 3 * width * height],
    }
}

impl Canvas {
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        let i = self.width * y + x;
        let r = self.data[i + 0] as f32 / 255.0;
        let g = self.data[i + 1] as f32 / 255.0;
        let b = self.data[i + 2] as f32 / 255.0;

        color(r, g, b)
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: &Color) {
        let r = (color.r * 255.0).round() as u8;
        let g = (color.g * 255.0).round() as u8;
        let b = (color.b * 255.0).round() as u8;
        let i = self.width * y + x;

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
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for component in c.data {
            assert_eq!(component, 0);
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);
        c.set_color(2, 3, &red);
        assert_eq!(c.get_color(2, 3), red);
    }
}
