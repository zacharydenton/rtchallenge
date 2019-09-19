use crate::color::*;
use crate::matrix::*;
use crate::object::*;
use crate::tuple::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternSpec {
    Stripe(Color, Color),
    LinearGradient(Color, Color),
    RadialGradient(Color, Color),
    Ring(Color, Color),
    Checkers(Color, Color),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    pub spec: PatternSpec,
    pub transform: Matrix4,
}

impl Pattern {
    /// Returns the pattern's color at the given point in world space.
    pub fn at(&self, point: Tuple4) -> Color {
        match self.spec {
            PatternSpec::Stripe(a, b) => {
                if point.x.floor() as i32 % 2 == 0 {
                    a
                } else {
                    b
                }
            }
            PatternSpec::LinearGradient(a, b) => {
                let distance = b - a;
                let fraction = point.x.fract();
                a + distance * fraction
            }
            PatternSpec::Ring(a, b) => {
                if (point.x * point.x + point.z * point.z).sqrt().floor() as i32 % 2 == 0 {
                    a
                } else {
                    b
                }
            }
            PatternSpec::Checkers(a, b) => {
                let c = point.x.floor() + point.z.floor();
                let f = (c * 0.5).fract();
                if f.abs() < 1e-2 {
                    a
                } else {
                    b
                }
            }
            PatternSpec::RadialGradient(a, b) => {
                let distance = b - a;
                let fraction = (point.x * point.x + point.z * point.z).sqrt().fract();
                a + distance * fraction
            }
        }
    }

    /// Returns the pattern's color on the given object at the given point in world space.
    pub fn at_object(&self, object: &Object, point: Tuple4) -> Color {
        let object_point = object.transform.inverse() * point;
        let pattern_point = self.transform.inverse() * object_point;
        self.at(pattern_point)
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        spec: PatternSpec::Stripe(a, b),
        transform: I4,
    }
}

pub fn linear_gradient_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        spec: PatternSpec::LinearGradient(a, b),
        transform: I4,
    }
}

pub fn ring_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        spec: PatternSpec::Ring(a, b),
        transform: I4,
    }
}

pub fn checkers_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        spec: PatternSpec::Checkers(a, b),
        transform: I4,
    }
}

pub fn radial_gradient_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        spec: PatternSpec::RadialGradient(a, b),
        transform: I4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::*;
    use assert_approx_eq::assert_approx_eq;

    fn white() -> Color {
        color(1., 1., 1.)
    }

    fn black() -> Color {
        color(0., 0., 0.)
    }

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = stripe_pattern(white(), black());
        if let PatternSpec::Stripe(a, b) = pattern.spec {
            assert_eq!(a, white());
            assert_eq!(b, black());
        } else {
            panic!();
        }
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = stripe_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0., 1., 0.)), white());
        assert_eq!(pattern.at(point3(0., 2., 0.)), white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0., 0., 1.)), white());
        assert_eq!(pattern.at(point3(0., 0., 2.)), white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0.9, 0., 0.)), white());
        assert_eq!(pattern.at(point3(1.0, 0., 0.)), black());
        assert_eq!(pattern.at(point3(-0.1, 0., 0.)), black());
        assert_eq!(pattern.at(point3(-1.0, 0., 0.)), black());
        assert_eq!(pattern.at(point3(-1.1, 0., 0.)), white());
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = sphere();
        object.transform = scale(2., 2., 2.);
        let pattern = stripe_pattern(white(), black());
        let c = pattern.at_object(&object, point3(1.5, 0., 0.));
        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = sphere();
        let mut pattern = stripe_pattern(white(), black());
        pattern.transform = scale(2., 2., 2.);
        let c = pattern.at_object(&object, point3(1.5, 0., 0.));
        assert_eq!(c, white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = sphere();
        object.transform = scale(2., 2., 2.);
        let mut pattern = stripe_pattern(white(), black());
        pattern.transform = translate(0.5, 0., 0.);
        let c = pattern.at_object(&object, point3(2.5, 0., 0.));
        assert_eq!(c, white());
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = linear_gradient_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0.25, 0., 0.)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern.at(point3(0.5, 0., 0.)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern.at(point3(0.75, 0., 0.)), color(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = ring_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(1., 0., 0.)), black());
        assert_eq!(pattern.at(point3(0., 0., 1.)), black());
        assert_eq!(pattern.at(point3(0.708, 0., 0.708)), black());
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = checkers_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0.99, 0., 0.)), white());
        assert_eq!(pattern.at(point3(1.01, 0., 0.)), black());
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = checkers_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0., 0.99, 0.)), white());
        assert_eq!(pattern.at(point3(0., 1.01, 0.)), black());
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = checkers_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0., 0., 0.99)), white());
        assert_eq!(pattern.at(point3(0., 0., 1.01)), black());
    }

    #[test]
    fn a_radial_gradient_should_interpolate_in_both_x_and_z() {
        let pattern = radial_gradient_pattern(white(), black());
        assert_eq!(pattern.at(point3(0., 0., 0.)), white());
        assert_eq!(pattern.at(point3(0.25, 0., 0.)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern.at(point3(0.5, 0., 0.)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern.at(point3(0.75, 0., 0.)), color(0.25, 0.25, 0.25));
        assert_eq!(pattern.at(point3(0., 0., 0.25)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern.at(point3(0., 0., 0.5)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern.at(point3(0., 0., 0.75)), color(0.25, 0.25, 0.25));

        let c1 = pattern.at(point3(
            0.25 * std::f32::consts::FRAC_1_SQRT_2,
            0.,
            0.25 * std::f32::consts::FRAC_1_SQRT_2,
        ));
        let c2 = pattern.at(point3(
            0.5 * std::f32::consts::FRAC_1_SQRT_2,
            0.,
            0.5 * std::f32::consts::FRAC_1_SQRT_2,
        ));
        let c3 = pattern.at(point3(
            0.75 * std::f32::consts::FRAC_1_SQRT_2,
            0.,
            0.75 * std::f32::consts::FRAC_1_SQRT_2,
        ));
        let c4 = pattern.at(point3(
            std::f32::consts::FRAC_1_SQRT_2,
            0.,
            std::f32::consts::FRAC_1_SQRT_2,
        ));

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
