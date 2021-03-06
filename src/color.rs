use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    pub const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }

    /// Clamps each color component to the range [0, 1].
    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.).max(0.),
            g: self.g.min(1.).max(0.),
            b: self.b.min(1.).max(0.),
        }
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl ops::Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 + c2;
        assert_approx_eq!(res.r, 1.6);
        assert_approx_eq!(res.g, 0.7);
        assert_approx_eq!(res.b, 1.0);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 - c2;
        assert_approx_eq!(res.r, 0.2);
        assert_approx_eq!(res.g, 0.5);
        assert_approx_eq!(res.b, 0.5);
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let res = c * 2.0;
        assert_approx_eq!(res.r, 0.4);
        assert_approx_eq!(res.g, 0.6);
        assert_approx_eq!(res.b, 0.8);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let res = c1 * c2;
        assert_approx_eq!(res.r, 0.9);
        assert_approx_eq!(res.g, 0.2);
        assert_approx_eq!(res.b, 0.04);
    }
}
