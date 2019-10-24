use crate::texture::*;

pub fn evaluate<T>(point: Tuple4, a: T, b: T) -> T {
    if (point.x * point.x + point.z * point.z).sqrt().floor() % 2. == 0. {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        assert_eq!(
            evaluate(point3(0., 0., 0.), Color::WHITE, Color::BLACK),
            Color::WHITE
        );
        assert_eq!(
            evaluate(point3(1., 0., 0.), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
        assert_eq!(
            evaluate(point3(0., 0., 1.), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
        assert_eq!(
            evaluate(point3(0.708, 0., 0.708), Color::WHITE, Color::BLACK),
            Color::BLACK
        );
    }
}
