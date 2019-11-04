use std::ops;

/// A 4-element tuple, used for representing points and vectors.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tuple4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// Constructs a Tuple4.
pub fn tuple4(x: f32, y: f32, z: f32, w: f32) -> Tuple4 {
    Tuple4 { x, y, z, w }
}

/// Constructs a Tuple4 with w = 1.0 (aka a point).
pub fn point3(x: f32, y: f32, z: f32) -> Tuple4 {
    Tuple4 { x, y, z, w: 1.0 }
}

/// Constructs a Tuple4 with w = 0.0 (aka a vector).
pub fn vector3(x: f32, y: f32, z: f32) -> Tuple4 {
    Tuple4 { x, y, z, w: 0.0 }
}

impl Tuple4 {
    /// Whether the Tuple4 is a point (w = 1.0).
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// Whether the Tuple4 is a vector (w = 0.0).
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// The distance represented by the tuple.
    pub fn magnitude(&self) -> f32 {
        (self.x.mul_add(
            self.x,
            self.y
                .mul_add(self.y, self.z.mul_add(self.z, self.w * self.w)),
        ))
        .sqrt()
    }

    /// Returns a normalized (magnitude = 1.0) form of the tuple.
    pub fn normalize(&self) -> Tuple4 {
        let m = self.magnitude();

        if m == 0. {
            // Prevent divide-by-zero.
            return *self;
        }

        Tuple4 {
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
    pub fn dot(&self, other: Tuple4) -> f32 {
        debug_assert!(self.is_vector());
        debug_assert!(other.is_vector());

        self.x
            .mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
    }

    /// Returns the cross product (aka vector product) with another vector.
    ///
    /// This is a new vector that is perpendicular to both of the original
    /// vectors.
    pub fn cross(&self, other: Tuple4) -> Tuple4 {
        debug_assert!(self.is_vector());
        debug_assert!(other.is_vector());

        Tuple4 {
            x: self.y.mul_add(other.z, -self.z * other.y),
            y: self.z.mul_add(other.x, -self.x * other.z),
            z: self.x.mul_add(other.y, -self.y * other.x),
            w: 0.,
        }
    }

    /// Returns a new vector representing this vector reflected around the
    /// normal.
    pub fn reflect(&self, normal: Tuple4) -> Tuple4 {
        *self - normal * 2. * self.dot(normal)
    }
}

impl ops::Add for Tuple4 {
    type Output = Tuple4;

    fn add(self, other: Tuple4) -> Tuple4 {
        Tuple4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub for Tuple4 {
    type Output = Tuple4;

    fn sub(self, other: Tuple4) -> Tuple4 {
        Tuple4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Neg for Tuple4 {
    type Output = Tuple4;

    fn neg(self) -> Tuple4 {
        Tuple4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f32> for Tuple4 {
    type Output = Tuple4;

    fn mul(self, other: f32) -> Tuple4 {
        Tuple4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Div<f32> for Tuple4 {
    type Output = Tuple4;

    fn div(self, other: f32) -> Tuple4 {
        Tuple4 {
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
        let a = tuple4(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = tuple4(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = point3(4., -4., 3.);
        assert_eq!(p, tuple4(4., -4., 3., 1.));
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn vector_creates_tuples_with_w_0() {
        let p = vector3(4., -4., 3.);
        assert_eq!(p, tuple4(4., -4., 3., 0.));
        assert!(!p.is_point());
        assert!(p.is_vector());
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = tuple4(3., -2., 5., 1.);
        let a2 = tuple4(-2., 3., 1., 0.);
        assert_eq!(a1 + a2, tuple4(1., 1., 6., 1.));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point3(3., 2., 1.);
        let p2 = point3(5., 6., 7.);
        assert_eq!(p1 - p2, vector3(-2., -4., -6.));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point3(3., 2., 1.);
        let v = vector3(5., 6., 7.);
        assert_eq!(p - v, point3(-2., -4., -6.));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector3(3., 2., 1.);
        let v2 = vector3(5., 6., 7.);
        assert_eq!(v1 - v2, vector3(-2., -4., -6.));
    }

    #[test]
    fn negating_a_tuple() {
        let a = tuple4(1., -2., 3., -4.);
        assert_eq!(-a, tuple4(-1., 2., -3., 4.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = tuple4(1., -2., 3., -4.);
        assert_eq!(a * 3.5, tuple4(3.5, -7., 10.5, -14.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = tuple4(1., -2., 3., -4.);
        assert_eq!(a * 0.5, tuple4(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = tuple4(1., -2., 3., -4.);
        assert_eq!(a / 2., tuple4(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn magnitude_of_vectors() {
        assert_eq!(vector3(1., 0., 0.).magnitude(), 1.);
        assert_eq!(vector3(0., 1., 0.).magnitude(), 1.);
        assert_eq!(vector3(0., 0., 1.).magnitude(), 1.);
        assert_eq!(vector3(1., 2., 3.).magnitude(), 14_f32.sqrt());
        assert_eq!(vector3(-1., -2., -3.).magnitude(), 14_f32.sqrt());
    }

    #[test]
    fn normalizing_vectors() {
        assert_eq!(vector3(4., 0., 0.).normalize(), vector3(1., 0., 0.));
        assert_eq!(
            vector3(1., 2., 3.).normalize(),
            vector3(1. / 14_f32.sqrt(), 2. / 14_f32.sqrt(), 3. / 14_f32.sqrt(),)
        );
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = vector3(1., 2., 3.);
        let norm = v.normalize();
        assert_approx_eq!(norm.magnitude(), 1.);
    }

    #[test]
    fn the_dot_product_of_two_vectors() {
        let a = vector3(1., 2., 3.);
        let b = vector3(2., 3., 4.);
        assert_eq!(a.dot(b), 20.);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = vector3(1., 2., 3.);
        let b = vector3(2., 3., 4.);
        assert_eq!(a.cross(b), vector3(-1., 2., -1.));
        assert_eq!(b.cross(a), vector3(1., -2., 1.));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = vector3(1., -1., 0.);
        let n = vector3(0., 1., 0.);
        let r = v.reflect(n);
        assert_approx_eq!(r.x, 1.);
        assert_approx_eq!(r.y, 1.);
        assert_approx_eq!(r.z, 0.);
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = vector3(0., -1., 0.);
        let root2over2 = std::f32::consts::SQRT_2 / 2.;
        let n = vector3(root2over2, root2over2, 0.);
        let r = v.reflect(n);
        assert_approx_eq!(r.x, 1.);
        assert_approx_eq!(r.y, 0.);
        assert_approx_eq!(r.z, 0.);
    }
}
