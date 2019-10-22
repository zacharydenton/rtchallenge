use crate::texture::*;

pub fn evaluate<T>(point: Tuple4, a: T, b: T) -> T {
    let c = point.x.floor() + point.z.floor();
    let f = (c * 0.5).fract();
    if f.abs() < 1e-3 {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkerboard_2d_should_repeat_in_x() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0.99, 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(1.01, 0., 0.), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
    }

    #[test]
    fn checkerboard_2d_should_not_repeat_in_y() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 0.99, 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 1.01, 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
    }

    #[test]
    fn checkerboard_2d_should_repeat_in_z() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 0., 0.99), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 0., 1.01), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
    }
}
