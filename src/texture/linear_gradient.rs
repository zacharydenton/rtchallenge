use crate::texture::*;
use std::ops::*;

pub fn evaluate<T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy>(
    point: Tuple4,
    a: T,
    b: T,
) -> T {
    let distance = b - a;
    let fraction = point.x.fract();
    a + distance * fraction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0.25, 0., 0.), Color::WHITE, Color::BLACK),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            evaluate(point3(0.5, 0., 0.), Color::WHITE, Color::BLACK),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            evaluate(point3(0.75, 0., 0.), Color::WHITE, Color::BLACK),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
