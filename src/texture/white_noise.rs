use crate::texture::*;
use std::ops::*;

pub fn evaluate<T: Mul<f32, Output = T>, R: Rng>(rng: &mut R, factor: T) -> T {
    factor * rng.gen::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn white_noise_is_random() {
        let mut rng = SmallRng::seed_from_u64(0);
        let green = Color::new(0., 1., 0.);

        let a = evaluate(&mut rng, green);
        let b = evaluate(&mut rng, green);
        let c = evaluate(&mut rng, green);
        let d = evaluate(&mut rng, green);

        assert_eq!(a, Color::new(0., 0.251_921_42, 0.));
        assert_eq!(b, Color::new(0., 0.913_606_3, 0.));
        assert_eq!(c, Color::new(0., 0.434_478_04, 0.));
        assert_eq!(d, Color::new(0., 0.092_519_58, 0.));
    }
}
