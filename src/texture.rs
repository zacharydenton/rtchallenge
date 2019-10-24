use crate::color::*;
use crate::transform::*;
use crate::tuple::*;
use rand::Rng;

pub mod checkerboard_2d;
pub mod checkerboard_3d;
pub mod linear_gradient;
pub mod radial_gradient;
pub mod ring;
pub mod stripe;
pub mod white_noise;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextureSpec {
    Constant(Color),
    Stripe(Color, Color),
    LinearGradient(Color, Color),
    RadialGradient(Color, Color),
    Ring(Color, Color),
    Checkerboard2D(Color, Color),
    Checkerboard3D(Color, Color),
    WhiteNoise,
    TestPattern,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Texture {
    pub spec: TextureSpec,
    pub transform: Transform,
}

impl Texture {
    pub fn constant(color: Color) -> Self {
        Texture {
            spec: TextureSpec::Constant(color),
            transform: Transform::new(),
        }
    }

    pub fn stripe(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::Stripe(a, b),
            transform: Transform::new(),
        }
    }

    pub fn linear_gradient(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::LinearGradient(a, b),
            transform: Transform::new(),
        }
    }

    pub fn radial_gradient(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::RadialGradient(a, b),
            transform: Transform::new(),
        }
    }

    pub fn ring(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::Ring(a, b),
            transform: Transform::new(),
        }
    }

    pub fn checkerboard_2d(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::Checkerboard2D(a, b),
            transform: Transform::new(),
        }
    }

    pub fn checkerboard_3d(a: Color, b: Color) -> Self {
        Texture {
            spec: TextureSpec::Checkerboard3D(a, b),
            transform: Transform::new(),
        }
    }

    pub fn white_noise() -> Self {
        Texture {
            spec: TextureSpec::WhiteNoise,
            transform: Transform::new(),
        }
    }

    pub fn test_pattern() -> Self {
        Texture {
            spec: TextureSpec::TestPattern,
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
        if let TextureSpec::Constant(color) = self.spec {
            // Skip transformations if the texture is constant everywhere.
            color
        } else {
            let object_point = object_transform.world_to_local * world_point;
            let texture_point = self.transform.world_to_local * object_point;
            self.evaluate_local(rng, texture_point)
        }
    }

    /// Returns the color at the given point in texture space.
    pub fn evaluate_local<R: Rng>(&self, rng: &mut R, texture_point: Tuple4) -> Color {
        match self.spec {
            TextureSpec::Constant(color) => color,
            TextureSpec::Stripe(a, b) => stripe::evaluate(texture_point, a, b),
            TextureSpec::LinearGradient(a, b) => linear_gradient::evaluate(texture_point, a, b),
            TextureSpec::RadialGradient(a, b) => radial_gradient::evaluate(texture_point, a, b),
            TextureSpec::Ring(a, b) => ring::evaluate(texture_point, a, b),
            TextureSpec::Checkerboard2D(a, b) => checkerboard_2d::evaluate(texture_point, a, b),
            TextureSpec::Checkerboard3D(a, b) => checkerboard_3d::evaluate(texture_point, a, b),
            TextureSpec::WhiteNoise => white_noise::evaluate(rng, Color::WHITE),
            TextureSpec::TestPattern => {
                Color::new(texture_point.x, texture_point.y, texture_point.z)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    use test::Bencher;

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

    #[bench]
    fn bench_evaluate_constant_texture(bencher: &mut Bencher) {
        let mut rng = SmallRng::seed_from_u64(0);
        let texture = Texture::constant(Color::WHITE);
        let transform = Transform::new().scale(2., 2., 2.);
        let point = point3(2.5, 0., 0.);

        bencher.iter(|| texture.evaluate(&mut rng, transform, point));
    }

    #[bench]
    fn bench_evaluate_stripe_texture(bencher: &mut Bencher) {
        let mut rng = SmallRng::seed_from_u64(0);
        let texture = Texture::stripe(Color::WHITE, Color::BLACK);
        let transform = Transform::new().scale(2., 2., 2.);
        let point = point3(2.5, 0., 0.);

        bencher.iter(|| texture.evaluate(&mut rng, transform, point));
    }
}
