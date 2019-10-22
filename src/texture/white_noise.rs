use crate::texture::*;

pub fn evaluate<R: Rng>(rng: &mut R) -> f32 {
    rng.gen::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn white_noise_is_random() {
        let mut rng = SmallRng::seed_from_u64(0);

        let a = evaluate(&mut rng);
        let b = evaluate(&mut rng);
        let c = evaluate(&mut rng);
        let d = evaluate(&mut rng);

        assert_eq!(a, 0.25192142);
        assert_eq!(b, 0.9136063);
        assert_eq!(c, 0.43447804);
        assert_eq!(d, 0.09251958);
    }
}
