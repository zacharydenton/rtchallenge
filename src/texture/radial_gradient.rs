use crate::texture::*;
use std::ops::*;

pub fn evaluate<T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy>(
    point: Tuple4,
    a: T,
    b: T,
) -> T {
    let distance = b - a;
    let fraction = (point.x * point.x + point.z * point.z).sqrt().fract();
    a + distance * fraction
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn a_radial_gradient_should_interpolate_in_both_x_and_z() {
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
        assert_eq!(
            evaluate(point3(0., 0., 0.25), Color::WHITE, Color::BLACK),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            evaluate(point3(0., 0., 0.5), Color::WHITE, Color::BLACK),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            evaluate(point3(0., 0., 0.75), Color::WHITE, Color::BLACK),
            Color::new(0.25, 0.25, 0.25)
        );

        let c1 = evaluate(
            point3(
                0.25 * std::f32::consts::FRAC_1_SQRT_2,
                0.,
                0.25 * std::f32::consts::FRAC_1_SQRT_2,
            ),
            Color::WHITE,
            Color::BLACK,
        );
        let c2 = evaluate(
            point3(
                0.5 * std::f32::consts::FRAC_1_SQRT_2,
                0.,
                0.5 * std::f32::consts::FRAC_1_SQRT_2,
            ),
            Color::WHITE,
            Color::BLACK,
        );
        let c3 = evaluate(
            point3(
                0.75 * std::f32::consts::FRAC_1_SQRT_2,
                0.,
                0.75 * std::f32::consts::FRAC_1_SQRT_2,
            ),
            Color::WHITE,
            Color::BLACK,
        );
        let c4 = evaluate(
            point3(
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
            ),
            Color::WHITE,
            Color::BLACK,
        );

        assert_approx_eq!(c1.r, 0.75);
        assert_approx_eq!(c1.g, 0.75);
        assert_approx_eq!(c1.b, 0.75);

        assert_approx_eq!(c2.r, 0.5);
        assert_approx_eq!(c2.g, 0.5);
        assert_approx_eq!(c2.b, 0.5);

        assert_approx_eq!(c3.r, 0.25);
        assert_approx_eq!(c3.g, 0.25);
        assert_approx_eq!(c3.b, 0.25);

        assert_approx_eq!(c4.r, 0.);
        assert_approx_eq!(c4.g, 0.);
        assert_approx_eq!(c4.b, 0.);
    }
}
