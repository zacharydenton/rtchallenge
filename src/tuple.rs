use std::ops;

/// A tuple is just an ordered list of things, like numbers.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Constructs a Tuple.
pub fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    Tuple { x, y, z, w }
}

/// Constructs a Tuple with w = 1.0 (aka a point).
pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

/// Constructs a Tuple with w = 0.0 (aka a vector).
pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

impl Tuple {
    /// Whether the Tuple is a point (w = 1.0).
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Whether the Tuple is a vector (w = 0.0).
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// The distance represented by the tuple.
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// Returns a normalized (magnitude = 1.0) form of the tuple.
    pub fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        Tuple {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    /// Returns the dot product (aka scalar product) with another vector.
    ///
    /// The smaller the dot product, the larger the angle between the
    /// vectors. For example, given two unit vectors, a dot product of 1 means
    /// the vectors are identical, and a dot product of -1 means they point in
    /// opposite directions.
    ///
    /// If the two vectors are unit vectors, the dot product is the cosine of
    /// the angle between them.
    pub fn dot(&self, other: &Tuple) -> f32 {
        debug_assert!(self.is_vector());
        debug_assert!(other.is_vector());

        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product (aka vector product) with another vector.
    ///
    /// This is a new vector that is perpendicular to both of the original vectors.
    pub fn cross(&self, other: &Tuple) -> Tuple {
        debug_assert!(self.is_vector());
        debug_assert!(other.is_vector());

        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.,
        }
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = point(4., -4., 3.);
        assert_eq!(p, tuple(4., -4., 3., 1.));
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn vector_creates_tuples_with_w_0() {
        let p = vector(4., -4., 3.);
        assert_eq!(p, tuple(4., -4., 3., 0.));
        assert!(!p.is_point());
        assert!(p.is_vector());
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = tuple(3., -2., 5., 1.);
        let a2 = tuple(-2., 3., 1., 0.);
        assert_eq!(a1 + a2, tuple(1., 1., 6., 1.));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3., 2., 1.);
        let p2 = point(5., 6., 7.);
        assert_eq!(p1 - p2, vector(-2., -4., -6.));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point(3., 2., 1.);
        let v = vector(5., 6., 7.);
        assert_eq!(p - v, point(-2., -4., -6.));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3., 2., 1.);
        let v2 = vector(5., 6., 7.);
        assert_eq!(v1 - v2, vector(-2., -4., -6.));
    }

    #[test]
    fn negating_a_tuple() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(-a, tuple(-1., 2., -3., 4.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(a * 3.5, tuple(3.5, -7., 10.5, -14.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(a * 0.5, tuple(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = tuple(1., -2., 3., -4.);
        assert_eq!(a / 2., tuple(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn magnitude_of_vectors() {
        assert_eq!(vector(1., 0., 0.).magnitude(), 1.);
        assert_eq!(vector(0., 1., 0.).magnitude(), 1.);
        assert_eq!(vector(0., 0., 1.).magnitude(), 1.);
        assert_eq!(vector(1., 2., 3.).magnitude(), (14 as f32).sqrt());
        assert_eq!(vector(-1., -2., -3.).magnitude(), (14 as f32).sqrt());
    }

    #[test]
    fn normalizing_vectors() {
        assert_eq!(vector(4., 0., 0.).normalize(), vector(1., 0., 0.));
        assert_eq!(
            vector(1., 2., 3.).normalize(),
            vector(
                1. / (14 as f32).sqrt(),
                2. / (14 as f32).sqrt(),
                3. / (14 as f32).sqrt(),
            )
        );
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = vector(1., 2., 3.);
        let norm = v.normalize();
        assert_approx_eq!(norm.magnitude(), 1.);
    }

    #[test]
    fn the_dot_product_of_two_vectors() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);
        assert_eq!(a.dot(&b), 20.);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);
        assert_eq!(a.cross(&b), vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), vector(1., -2., 1.));
    }
}
