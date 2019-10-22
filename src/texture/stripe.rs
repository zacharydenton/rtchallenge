use crate::texture::*;

pub fn evaluate<T>(point: Tuple4, a: T, b: T) -> T {
    if point.x.floor() as i32 % 2 == 0 {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 1., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 2., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 0., 1.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0., 0., 2.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(0.9, 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(1.0, 0., 0.), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
        assert_eq!(
            evaluate(point3(-0.1, 0., 0.), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
        assert_eq!(
            evaluate(point3(-1.0, 0., 0.), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
        assert_eq!(
            evaluate(point3(-1.1, 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
    }
}
