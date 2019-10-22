use crate::color::*;
use crate::transform::*;
use crate::tuple::*;
use rand::Rng;
use std::ops;

pub mod checkerboard_2d;
pub mod checkerboard_3d;
pub mod ring;
pub mod stripe;
pub mod white_noise;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureSpec<'a> {
    Constant(Color),
    Stripe(&'a Texture<'a>, &'a Texture<'a>),
    Ring(&'a Texture<'a>, &'a Texture<'a>),
    Checkerboard2D(&'a Texture<'a>, &'a Texture<'a>),
    Checkerboard3D(&'a Texture<'a>, &'a Texture<'a>),
    WhiteNoise(&'a Texture<'a>),
    Add(&'a Texture<'a>, &'a Texture<'a>),
    Subtract(&'a Texture<'a>, &'a Texture<'a>),
    Scale(&'a Texture<'a>, f32),
    TestPattern,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Texture<'a> {
    pub spec: TextureSpec<'a>,
    pub transform: Transform,
}

impl<'a> Texture<'a> {
    pub fn constant(color: Color) -> Self {
        Texture {
            spec: TextureSpec::Constant(color),
            transform: Transform::new(),
        }
    }

    pub fn stripe(a: &'a Texture, b: &'a Texture) -> Self {
        Texture {
            spec: TextureSpec::Stripe(a, b),
            transform: Transform::new(),
        }
    }

    /*
    pub fn linear_gradient(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::LinearGradient(&Texture::constant(a), &Texture::constant(b)),
            transform: Transform::new(),
        }
    }

    pub fn radial_gradient(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::RadialGradient(&Texture::constant(a), &Texture::constant(b)),
            transform: Transform::new(),
        }
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::Ring(&Texture::constant(a), &Texture::constant(b)),
            transform: Transform::new(),
        }
    }

    pub fn checkerboard_2d(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::Checkerboard2D(&Texture::constant(a), &Texture::constant(b)),
            transform: Transform::new(),
        }
    }

    pub fn checkerboard_3d(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::Checkerboard3D(&Texture::constant(a), &Texture::constant(b)),
            transform: Transform::new(),
        }
    }

    pub fn white_noise() -> Self {
        Texture {
            spec: TextureSpec::WhiteNoise(&Texture::constant(Color::WHITE)),
            transform: Transform::new(),
        }
    }

    pub fn test_pattern() -> Self {
        Texture {
            spec: TextureSpec::TestPattern,
            transform: Transform::new(),
        }
    }
    */

    pub fn add(a: &'a Texture<'a>, b: &'a Texture<'a>) -> Self {
        Texture {
            spec: TextureSpec::Add(a, b),
            transform: Transform::new(),
        }
    }

    pub fn subtract(a: &'a Texture<'a>, b: &'a Texture<'a>) -> Self {
        Texture {
            spec: TextureSpec::Subtract(a, b),
            transform: Transform::new(),
        }
    }

    pub fn scale(texture: &'a Texture<'a>, amount: f32) -> Self {
        Texture {
            spec: TextureSpec::Scale(texture, amount),
            transform: Transform::new(),
        }
    }

    /// Returns the color at the given point in world space.
    pub fn evaluate<R: Rng>(
        &self,
        rng: &mut R,
        object_transform: Transform,
        world_point: Tuple4,
    ) -> Color {
        let object_point = object_transform.world_to_local * world_point;
        let texture_point = self.transform.world_to_local * object_point;
        self.evaluate_local(rng, texture_point)
    }

    /// Returns the color at the given point in texture space.
    pub fn evaluate_local<R: Rng>(&self, rng: &mut R, texture_point: Tuple4) -> Color {
        match self.spec {
            TextureSpec::Constant(color) => color,
            TextureSpec::Add(a, b) => {
                let color_a = a.evaluate_local(rng, texture_point);
                let color_b = b.evaluate_local(rng, texture_point);
                color_a + color_b
            }
            TextureSpec::Subtract(a, b) => {
                let color_a = a.evaluate_local(rng, texture_point);
                let color_b = b.evaluate_local(rng, texture_point);
                color_a - color_b
            }
            TextureSpec::Scale(texture, amount) => {
                let color = texture.evaluate_local(rng, texture_point);
                color * amount
            }
            TextureSpec::TestPattern => {
                Color::new(texture_point.x, texture_point.y, texture_point.z)
            }
            _ => {
                let result = self.evaluate_procedural(rng, texture_point);
                result.evaluate_local(rng, texture_point)
            }
        }
    }

    fn evaluate_procedural<R: Rng>(&self, rng: &mut R, texture_point: Tuple4) -> &'a Texture {
        match self.spec {
            TextureSpec::Stripe(a, b) => stripe::evaluate(texture_point, a, b),
            TextureSpec::Ring(a, b) => ring::evaluate(texture_point, a, b),
            TextureSpec::Checkerboard2D(a, b) => checkerboard_2d::evaluate(texture_point, a, b),
            TextureSpec::Checkerboard3D(a, b) => checkerboard_3d::evaluate(texture_point, a, b),
            TextureSpec::WhiteNoise(texture) => &Texture::scale(texture, white_noise::evaluate(rng)),
            _ => panic!("Expected a procedural texture.")
        }
    }
}

impl<'a> ops::Add for &'a Texture<'a> {
    type Output = Texture<'a>;

    fn add(self, other: &'a Texture<'a>) -> Texture<'a> {
        Texture::add(self, other)
    }
}

impl<'a> ops::Sub for &'a Texture<'a> {
    type Output = Texture<'a>;

    fn sub(self, other: &'a Texture<'a>) -> Texture<'a> {
        Texture::subtract(self, other)
    }
}

impl<'a> ops::Mul<f32> for &'a Texture<'a> {
    type Output = Texture<'a>;

    fn mul(self, other: f32) -> Texture<'a> {
        Texture::scale(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn creating_a_stripe_texture() {
        let texture = Texture::stripe(Color::WHITE, Color::BLACK);
        assert_eq!(
            texture.spec,
            TextureSpec::Stripe(Color::WHITE, Color::BLACK)
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut rng = SmallRng::seed_from_u64(0);
        let texture = Texture::stripe(Color::WHITE, Color::BLACK);
        let c = texture.evaluate(
            &mut rng,
            Transform::new().scale(2., 2., 2.),
            point3(1.5, 0., 0.),
        );
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn stripes_with_a_texture_transformation() {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut texture = Texture::stripe(Color::WHITE, Color::BLACK);
        texture.transform.scale(2., 2., 2.);
        let c = texture.evaluate(&mut rng, Transform::new(), point3(1.5, 0., 0.));
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_texture_transformation() {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut texture = Texture::stripe(Color::WHITE, Color::BLACK);
        texture.transform.translate(0.5, 0., 0.);
        let c = texture.evaluate(
            &mut rng,
            Transform::new().scale(2., 2., 2.),
            point3(2.5, 0., 0.),
        );
        assert_eq!(c, Color::WHITE);
    }

    #[test]
    fn a_texture_with_an_object_transformation() {
        let mut rng = SmallRng::seed_from_u64(0);
        let texture = Texture::test_pattern();
        let c = texture.evaluate(
            &mut rng,
            Transform::new().scale(2., 2., 2.),
            point3(2., 3., 4.),
        );

        assert_approx_eq!(c.r, 1.);
        assert_approx_eq!(c.g, 1.5);
        assert_approx_eq!(c.b, 2.);
    }

    #[test]
    fn a_texture_with_a_texture_transformation() {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut texture = Texture::test_pattern();
        texture.transform.scale(2., 2., 2.);
        let c = texture.evaluate(&mut rng, Transform::new(), point3(2., 3., 4.));

        assert_approx_eq!(c.r, 1.);
        assert_approx_eq!(c.g, 1.5);
        assert_approx_eq!(c.b, 2.);
    }

    #[test]
    fn a_texture_with_an_object_transformation_and_a_texture_transformation() {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut texture = Texture::test_pattern();
        texture.transform.translate(0.5, 1., 1.5);
        let c = texture.evaluate(
            &mut rng,
            Transform::new().scale(2., 2., 2.),
            point3(2.5, 3., 3.5),
        );

        assert_approx_eq!(c.r, 0.75);
        assert_approx_eq!(c.g, 0.5);
        assert_approx_eq!(c.b, 0.25);
    }
}
