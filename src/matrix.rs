use crate::tuple::*;
use std::ops;

/// A 2x2 matrix.
///
/// | x0 | x1 |
/// | y0 | y1 |
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix2 {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

pub fn matrix2(x0: f32, x1: f32, y0: f32, y1: f32) -> Matrix2 {
    Matrix2 { x0, y0, x1, y1 }
}

impl Matrix2 {
    #[inline]
    pub fn determinant(&self) -> f32 {
        self.x0.mul_add(self.y1, -self.x1 * self.y0)
    }
}

pub const I2: Matrix2 = Matrix2 {
    x0: 1.,
    y0: 0.,
    x1: 0.,
    y1: 1.,
};

/// A 3x3 matrix.
///
/// | x0 | x1 | x2 |
/// | y0 | y1 | y2 |
/// | z0 | z1 | z2 |
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3 {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
    pub x2: f32,
    pub y2: f32,
    pub z2: f32,
}

pub fn matrix3(
    x0: f32,
    x1: f32,
    x2: f32,
    y0: f32,
    y1: f32,
    y2: f32,
    z0: f32,
    z1: f32,
    z2: f32,
) -> Matrix3 {
    Matrix3 {
        x0,
        y0,
        z0,
        x1,
        y1,
        z1,
        x2,
        y2,
        z2,
    }
}

impl Matrix3 {
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix2 {
        match (row, column) {
            (0, 0) => Matrix2 {
                x0: self.y1,
                x1: self.y2,
                y0: self.z1,
                y1: self.z2,
            },
            (0, 1) => Matrix2 {
                x0: self.y0,
                x1: self.y2,
                y0: self.z0,
                y1: self.z2,
            },
            (0, 2) => Matrix2 {
                x0: self.y0,
                x1: self.y1,
                y0: self.z0,
                y1: self.z1,
            },
            (1, 0) => Matrix2 {
                x0: self.x1,
                x1: self.x2,
                y0: self.z1,
                y1: self.z2,
            },
            (1, 1) => Matrix2 {
                x0: self.x0,
                x1: self.x2,
                y0: self.z0,
                y1: self.z2,
            },
            (1, 2) => Matrix2 {
                x0: self.x0,
                x1: self.x1,
                y0: self.z0,
                y1: self.z1,
            },
            (2, 0) => Matrix2 {
                x0: self.x1,
                x1: self.x2,
                y0: self.y1,
                y1: self.y2,
            },
            (2, 1) => Matrix2 {
                x0: self.x0,
                x1: self.x2,
                y0: self.y0,
                y1: self.y2,
            },
            (2, 2) => Matrix2 {
                x0: self.x0,
                x1: self.x1,
                y0: self.y0,
                y1: self.y1,
            },
            (_, _) => panic!("Invalid submatrix requested (row and column must be 0, 1, or 2)."),
        }
    }

    #[inline]
    pub fn minor(&self, row: usize, column: usize) -> f32 {
        match (row, column) {
            (0, 0) => (self.y1.mul_add(self.z2, -self.y2 * self.z1)),
            (0, 1) => (self.y0.mul_add(self.z2, -self.y2 * self.z0)),
            (0, 2) => (self.y0.mul_add(self.z1, -self.y1 * self.z0)),
            (1, 0) => (self.x1.mul_add(self.z2, -self.x2 * self.z1)),
            (1, 1) => (self.x0.mul_add(self.z2, -self.x2 * self.z0)),
            (1, 2) => (self.x0.mul_add(self.z1, -self.x1 * self.z0)),
            (2, 0) => (self.x1.mul_add(self.y2, -self.x2 * self.y1)),
            (2, 1) => (self.x0.mul_add(self.y2, -self.x2 * self.y0)),
            (2, 2) => (self.x0.mul_add(self.y1, -self.x1 * self.y0)),
            (_, _) => panic!("Invalid submatrix requested (row and column must be 0, 1, or 2)."),
        }
    }

    #[inline]
    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        match (row, column) {
            (0, 0) => (self.y1.mul_add(self.z2, -self.y2 * self.z1)),
            (0, 1) => -(self.y0.mul_add(self.z2, -self.y2 * self.z0)),
            (0, 2) => (self.y0.mul_add(self.z1, -self.y1 * self.z0)),
            (1, 0) => -(self.x1.mul_add(self.z2, -self.x2 * self.z1)),
            (1, 1) => (self.x0.mul_add(self.z2, -self.x2 * self.z0)),
            (1, 2) => -(self.x0.mul_add(self.z1, -self.x1 * self.z0)),
            (2, 0) => (self.x1.mul_add(self.y2, -self.x2 * self.y1)),
            (2, 1) => -(self.x0.mul_add(self.y2, -self.x2 * self.y0)),
            (2, 2) => (self.x0.mul_add(self.y1, -self.x1 * self.y0)),
            (_, _) => panic!("Invalid submatrix requested (row and column must be 0, 1, or 2)."),
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.x0.mul_add(
            self.y1.mul_add(self.z2, -self.y2 * self.z1),
            self.x2 * self.y0.mul_add(self.z1, -self.y1 * self.z0),
        ) - self.x1 * self.y0.mul_add(self.z2, -self.y2 * self.z0)
    }
}

pub const I3: Matrix3 = Matrix3 {
    x0: 1.,
    y0: 0.,
    z0: 0.,
    x1: 0.,
    y1: 1.,
    z1: 0.,
    x2: 0.,
    y2: 0.,
    z2: 1.,
};

/// A 4x4 matrix.
///
/// | x0 | x1 | x2 | x3 |
/// | y0 | y1 | y2 | y3 |
/// | z0 | z1 | z2 | z3 |
/// | w0 | w1 | w2 | w3 |
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4 {
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub w0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
    pub w1: f32,
    pub x2: f32,
    pub y2: f32,
    pub z2: f32,
    pub w2: f32,
    pub x3: f32,
    pub y3: f32,
    pub z3: f32,
    pub w3: f32,
}

pub fn matrix4(
    x0: f32,
    x1: f32,
    x2: f32,
    x3: f32,
    y0: f32,
    y1: f32,
    y2: f32,
    y3: f32,
    z0: f32,
    z1: f32,
    z2: f32,
    z3: f32,
    w0: f32,
    w1: f32,
    w2: f32,
    w3: f32,
) -> Matrix4 {
    Matrix4 {
        x0,
        y0,
        z0,
        w0,
        x1,
        y1,
        z1,
        w1,
        x2,
        y2,
        z2,
        w2,
        x3,
        y3,
        z3,
        w3,
    }
}

pub const I4: Matrix4 = Matrix4 {
    x0: 1.,
    y0: 0.,
    z0: 0.,
    w0: 0.,
    x1: 0.,
    y1: 1.,
    z1: 0.,
    w1: 0.,
    x2: 0.,
    y2: 0.,
    z2: 1.,
    w2: 0.,
    x3: 0.,
    y3: 0.,
    z3: 0.,
    w3: 1.,
};

impl Matrix4 {
    pub fn transpose(&self) -> Matrix4 {
        Matrix4 {
            x0: self.x0,
            y0: self.x1,
            z0: self.x2,
            w0: self.x3,
            x1: self.y0,
            y1: self.y1,
            z1: self.y2,
            w1: self.y3,
            x2: self.z0,
            y2: self.z1,
            z2: self.z2,
            w2: self.z3,
            x3: self.w0,
            y3: self.w1,
            z3: self.w2,
            w3: self.w3,
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix3 {
        match (row, column) {
            (0, 0) => Matrix3 {
                x0: self.y1,
                y0: self.z1,
                z0: self.w1,
                x1: self.y2,
                y1: self.z2,
                z1: self.w2,
                x2: self.y3,
                y2: self.z3,
                z2: self.w3,
            },
            (0, 1) => Matrix3 {
                x0: self.y0,
                y0: self.z0,
                z0: self.w0,
                x1: self.y2,
                y1: self.z2,
                z1: self.w2,
                x2: self.y3,
                y2: self.z3,
                z2: self.w3,
            },
            (0, 2) => Matrix3 {
                x0: self.y0,
                y0: self.z0,
                z0: self.w0,
                x1: self.y1,
                y1: self.z1,
                z1: self.w1,
                x2: self.y3,
                y2: self.z3,
                z2: self.w3,
            },
            (0, 3) => Matrix3 {
                x0: self.y0,
                y0: self.z0,
                z0: self.w0,
                x1: self.y1,
                y1: self.z1,
                z1: self.w1,
                x2: self.y2,
                y2: self.z2,
                z2: self.w2,
            },
            (1, 0) => Matrix3 {
                x0: self.x1,
                y0: self.z1,
                z0: self.w1,
                x1: self.x2,
                y1: self.z2,
                z1: self.w2,
                x2: self.x3,
                y2: self.z3,
                z2: self.w3,
            },
            (1, 1) => Matrix3 {
                x0: self.x0,
                y0: self.z0,
                z0: self.w0,
                x1: self.x2,
                y1: self.z2,
                z1: self.w2,
                x2: self.x3,
                y2: self.z3,
                z2: self.w3,
            },
            (1, 2) => Matrix3 {
                x0: self.x0,
                y0: self.z0,
                z0: self.w0,
                x1: self.x1,
                y1: self.z1,
                z1: self.w1,
                x2: self.x3,
                y2: self.z3,
                z2: self.w3,
            },
            (1, 3) => Matrix3 {
                x0: self.x0,
                y0: self.z0,
                z0: self.w0,
                x1: self.x1,
                y1: self.z1,
                z1: self.w1,
                x2: self.x2,
                y2: self.z2,
                z2: self.w2,
            },
            (2, 0) => Matrix3 {
                x0: self.x1,
                y0: self.y1,
                z0: self.w1,
                x1: self.x2,
                y1: self.y2,
                z1: self.w2,
                x2: self.x3,
                y2: self.y3,
                z2: self.w3,
            },
            (2, 1) => Matrix3 {
                x0: self.x0,
                y0: self.y0,
                z0: self.w0,
                x1: self.x2,
                y1: self.y2,
                z1: self.w2,
                x2: self.x3,
                y2: self.y3,
                z2: self.w3,
            },
            (2, 2) => Matrix3 {
                x0: self.x0,
                y0: self.y0,
                z0: self.w0,
                x1: self.x1,
                y1: self.y1,
                z1: self.w1,
                x2: self.x3,
                y2: self.y3,
                z2: self.w3,
            },
            (2, 3) => Matrix3 {
                x0: self.x0,
                y0: self.y0,
                z0: self.w0,
                x1: self.x1,
                y1: self.y1,
                z1: self.w1,
                x2: self.x2,
                y2: self.y2,
                z2: self.w2,
            },
            (3, 0) => Matrix3 {
                x0: self.x1,
                y0: self.y1,
                z0: self.z1,
                x1: self.x2,
                y1: self.y2,
                z1: self.z2,
                x2: self.x3,
                y2: self.y3,
                z2: self.z3,
            },
            (3, 1) => Matrix3 {
                x0: self.x0,
                y0: self.y0,
                z0: self.z0,
                x1: self.x2,
                y1: self.y2,
                z1: self.z2,
                x2: self.x3,
                y2: self.y3,
                z2: self.z3,
            },
            (3, 2) => Matrix3 {
                x0: self.x0,
                y0: self.y0,
                z0: self.z0,
                x1: self.x1,
                y1: self.y1,
                z1: self.z1,
                x2: self.x3,
                y2: self.y3,
                z2: self.z3,
            },
            (3, 3) => Matrix3 {
                x0: self.x0,
                y0: self.y0,
                z0: self.z0,
                x1: self.x1,
                y1: self.y1,
                z1: self.z1,
                x2: self.x2,
                y2: self.y2,
                z2: self.z2,
            },
            (_, _) => panic!("Invalid submatrix requested (row and column must be 0, 1, 2, or 3)."),
        }
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        if row + column & 1 == 1 {
            -self.minor(row, column)
        } else {
            self.minor(row, column)
        }
    }

    pub fn determinant(&self) -> f32 {
        self.x0
            .mul_add(self.minor(0, 0), self.x2 * self.minor(0, 2))
            - self
                .x1
                .mul_add(self.minor(0, 1), self.x3 * self.minor(0, 3))
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }

    pub fn inverse(&self) -> Matrix4 {
        debug_assert!(
            self.is_invertible(),
            "Attempted to invert a non-invertible matrix."
        );

        let c00 = self.minor(0, 0);
        let c01 = -self.minor(0, 1);
        let c02 = self.minor(0, 2);
        let c03 = -self.minor(0, 3);
        let inv_determinant = (self.x0.mul_add(
            c00,
            self.x1.mul_add(c01, self.x2.mul_add(c02, self.x3 * c03)),
        ))
        .recip();

        Matrix4 {
            x0: c00 * inv_determinant,
            y0: c01 * inv_determinant,
            z0: c02 * inv_determinant,
            w0: c03 * inv_determinant,
            x1: -self.minor(1, 0) * inv_determinant,
            y1: self.minor(1, 1) * inv_determinant,
            z1: -self.minor(1, 2) * inv_determinant,
            w1: self.minor(1, 3) * inv_determinant,
            x2: self.minor(2, 0) * inv_determinant,
            y2: -self.minor(2, 1) * inv_determinant,
            z2: self.minor(2, 2) * inv_determinant,
            w2: -self.minor(2, 3) * inv_determinant,
            x3: -self.minor(3, 0) * inv_determinant,
            y3: self.minor(3, 1) * inv_determinant,
            z3: -self.minor(3, 2) * inv_determinant,
            w3: self.minor(3, 3) * inv_determinant,
        }
    }
}

impl ops::Mul for Matrix4 {
    type Output = Matrix4;

    #[inline]
    fn mul(self, other: Matrix4) -> Matrix4 {
        Matrix4 {
            x0: self.x0.mul_add(
                other.x0,
                self.x1
                    .mul_add(other.y0, self.x2.mul_add(other.z0, self.x3 * other.w0)),
            ),
            y0: self.y0.mul_add(
                other.x0,
                self.y1
                    .mul_add(other.y0, self.y2.mul_add(other.z0, self.y3 * other.w0)),
            ),
            z0: self.z0.mul_add(
                other.x0,
                self.z1
                    .mul_add(other.y0, self.z2.mul_add(other.z0, self.z3 * other.w0)),
            ),
            w0: self.w0.mul_add(
                other.x0,
                self.w1
                    .mul_add(other.y0, self.w2.mul_add(other.z0, self.w3 * other.w0)),
            ),
            x1: self.x0.mul_add(
                other.x1,
                self.x1
                    .mul_add(other.y1, self.x2.mul_add(other.z1, self.x3 * other.w1)),
            ),
            y1: self.y0.mul_add(
                other.x1,
                self.y1
                    .mul_add(other.y1, self.y2.mul_add(other.z1, self.y3 * other.w1)),
            ),
            z1: self.z0.mul_add(
                other.x1,
                self.z1
                    .mul_add(other.y1, self.z2.mul_add(other.z1, self.z3 * other.w1)),
            ),
            w1: self.w0.mul_add(
                other.x1,
                self.w1
                    .mul_add(other.y1, self.w2.mul_add(other.z1, self.w3 * other.w1)),
            ),
            x2: self.x0.mul_add(
                other.x2,
                self.x1
                    .mul_add(other.y2, self.x2.mul_add(other.z2, self.x3 * other.w2)),
            ),
            y2: self.y0.mul_add(
                other.x2,
                self.y1
                    .mul_add(other.y2, self.y2.mul_add(other.z2, self.y3 * other.w2)),
            ),
            z2: self.z0.mul_add(
                other.x2,
                self.z1
                    .mul_add(other.y2, self.z2.mul_add(other.z2, self.z3 * other.w2)),
            ),
            w2: self.w0.mul_add(
                other.x2,
                self.w1
                    .mul_add(other.y2, self.w2.mul_add(other.z2, self.w3 * other.w2)),
            ),
            x3: self.x0.mul_add(
                other.x3,
                self.x1
                    .mul_add(other.y3, self.x2.mul_add(other.z3, self.x3 * other.w3)),
            ),
            y3: self.y0.mul_add(
                other.x3,
                self.y1
                    .mul_add(other.y3, self.y2.mul_add(other.z3, self.y3 * other.w3)),
            ),
            z3: self.z0.mul_add(
                other.x3,
                self.z1
                    .mul_add(other.y3, self.z2.mul_add(other.z3, self.z3 * other.w3)),
            ),
            w3: self.w0.mul_add(
                other.x3,
                self.w1
                    .mul_add(other.y3, self.w2.mul_add(other.z3, self.w3 * other.w3)),
            ),
        }
    }
}

impl ops::Mul<Tuple4> for Matrix4 {
    type Output = Tuple4;

    #[inline]
    fn mul(self, other: Tuple4) -> Tuple4 {
        Tuple4 {
            x: self.x0.mul_add(
                other.x,
                self.x1
                    .mul_add(other.y, self.x2.mul_add(other.z, self.x3 * other.w)),
            ),
            y: self.y0.mul_add(
                other.x,
                self.y1
                    .mul_add(other.y, self.y2.mul_add(other.z, self.y3 * other.w)),
            ),
            z: self.z0.mul_add(
                other.x,
                self.z1
                    .mul_add(other.y, self.z2.mul_add(other.z, self.z3 * other.w)),
            ),
            w: self.w0.mul_add(
                other.x,
                self.w1
                    .mul_add(other.y, self.w2.mul_add(other.z, self.w3 * other.w)),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use test::Bencher;

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let m = matrix2(-3., 5., 1., -2.);
        assert_eq!(m.x0, -3.);
        assert_eq!(m.x1, 5.);
        assert_eq!(m.y0, 1.);
        assert_eq!(m.y1, -2.);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let m = matrix3(-3., 5., 0., 1., -2., -7., 0., 1., 1.);
        assert_eq!(m.x0, -3.);
        assert_eq!(m.y1, -2.);
        assert_eq!(m.z2, 1.);
    }

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = matrix4(
            1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
        );
        assert_eq!(m.x0, 1.);
        assert_eq!(m.x3, 4.);
        assert_eq!(m.y0, 5.5);
        assert_eq!(m.y2, 7.5);
        assert_eq!(m.z2, 11.);
        assert_eq!(m.w0, 13.5);
        assert_eq!(m.w2, 15.5);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = matrix4(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        );
        let b = matrix4(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        );
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = matrix4(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        );
        let b = matrix4(
            2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = matrix4(
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        );
        let b = matrix4(
            -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
        );
        assert_eq!(
            a * b,
            matrix4(
                20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46., 42.,
            )
        );
    }

    #[test]
    fn multiplying_a_matrix_and_a_tuple() {
        let a = matrix4(
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        );
        let b = tuple4(1., 2., 3., 1.);
        assert_eq!(a * b, tuple4(18., 24., 33., 1.,));
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = matrix4(
            0., 1., 2., 4., 1., 2., 4., 8., 2., 4., 8., 16., 4., 8., 16., 32.,
        );
        assert_eq!(a * I4, a);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = tuple4(1., 2., 3., 4.);
        assert_eq!(I4 * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = matrix4(
            0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
        );
        assert_eq!(
            a.transpose(),
            matrix4(0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,)
        );
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = matrix2(1., 5., -3., 2.);
        assert_eq!(a.determinant(), 17.);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = matrix3(1., 5., 0., -3., 2., 7., 0., 6., -3.);
        assert_eq!(a.submatrix(0, 2), matrix2(-3., 2., 0., 6.,));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = matrix4(
            -6., 1., 1., 6., -8., 5., 8., 6., -1., 0., 8., 2., -7., 1., -1., 1.,
        );
        assert_eq!(
            a.submatrix(2, 1),
            matrix3(-6., 1., 6., -8., 8., 6., -7., -1., 1.,)
        );
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = matrix3(3., 5., 0., 2., -1., -7., 6., -1., 5.);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.);
        assert_eq!(a.minor(1, 0), 25.);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = matrix3(3., 5., 0., 2., -1., -7., 6., -1., 5.);
        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = matrix3(1., 2., 6., -5., 8., -4., 2., 6., 4.);
        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = matrix4(
            -2., -8., 3., 5., -3., 1., 7., 3., 1., 2., -9., 6., -6., 7., 7., -9.,
        );
        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = matrix4(
            6., 4., 4., 4., 5., 5., 7., 6., 4., -9., 3., -7., 9., 1., 7., -6.,
        );
        assert_eq!(a.determinant(), -2120.);
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = matrix4(
            -4., 2., -2., -3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., 0.,
        );
        assert_eq!(a.determinant(), 0.);
        assert!(!a.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = matrix4(
            -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
        );
        let b = a.inverse();

        assert_approx_eq!(a.determinant(), 532.);
        assert_approx_eq!(a.cofactor(2, 3), -160.);
        assert_approx_eq!(b.w2, -160. / 532.);
        assert_approx_eq!(a.cofactor(3, 2), 105.);
        assert_approx_eq!(b.z3, 105. / 532.);

        assert_approx_eq!(b.x0, 0.21805, 1.0e-5);
        assert_approx_eq!(b.x1, 0.45113, 1.0e-5);
        assert_approx_eq!(b.x2, 0.24060, 1.0e-5);
        assert_approx_eq!(b.x3, -0.04511, 1.0e-5);
        assert_approx_eq!(b.y0, -0.80827, 1.0e-5);
        assert_approx_eq!(b.y1, -1.45677, 1.0e-5);
        assert_approx_eq!(b.y2, -0.44361, 1.0e-5);
        assert_approx_eq!(b.y3, 0.52068, 1.0e-5);
        assert_approx_eq!(b.z0, -0.07895, 1.0e-5);
        assert_approx_eq!(b.z1, -0.22368, 1.0e-5);
        assert_approx_eq!(b.z2, -0.05263, 1.0e-5);
        assert_approx_eq!(b.z3, 0.19737, 1.0e-5);
        assert_approx_eq!(b.w0, -0.52256, 1.0e-5);
        assert_approx_eq!(b.w1, -0.81391, 1.0e-5);
        assert_approx_eq!(b.w2, -0.30075, 1.0e-5);
        assert_approx_eq!(b.w3, 0.30639, 1.0e-5);
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = matrix4(
            8., -5., 9., 2., 7., 5., 6., 1., -6., 0., 9., 6., -3., 0., -9., -4.,
        );
        let b = a.inverse();

        assert_approx_eq!(b.x0, -0.15385, 1.0e-5);
        assert_approx_eq!(b.x1, -0.15385, 1.0e-5);
        assert_approx_eq!(b.x2, -0.28205, 1.0e-5);
        assert_approx_eq!(b.x3, -0.53846, 1.0e-5);
        assert_approx_eq!(b.y0, -0.07692, 1.0e-5);
        assert_approx_eq!(b.y1, 0.12308, 1.0e-5);
        assert_approx_eq!(b.y2, 0.02564, 1.0e-5);
        assert_approx_eq!(b.y3, 0.03077, 1.0e-5);
        assert_approx_eq!(b.z0, 0.35897, 1.0e-5);
        assert_approx_eq!(b.z1, 0.35897, 1.0e-5);
        assert_approx_eq!(b.z2, 0.43590, 1.0e-5);
        assert_approx_eq!(b.z3, 0.92308, 1.0e-5);
        assert_approx_eq!(b.w0, -0.69231, 1.0e-5);
        assert_approx_eq!(b.w1, -0.69231, 1.0e-5);
        assert_approx_eq!(b.w2, -0.76923, 1.0e-5);
        assert_approx_eq!(b.w3, -1.92308, 1.0e-5);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a = matrix4(
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        );
        let b = a.inverse();

        assert_approx_eq!(b.x0, -0.04074, 1.0e-5);
        assert_approx_eq!(b.x1, -0.07778, 1.0e-5);
        assert_approx_eq!(b.x2, 0.14444, 1.0e-5);
        assert_approx_eq!(b.x3, -0.22222, 1.0e-5);
        assert_approx_eq!(b.y0, -0.07778, 1.0e-5);
        assert_approx_eq!(b.y1, 0.03333, 1.0e-5);
        assert_approx_eq!(b.y2, 0.36667, 1.0e-5);
        assert_approx_eq!(b.y3, -0.33333, 1.0e-5);
        assert_approx_eq!(b.z0, -0.02901, 1.0e-5);
        assert_approx_eq!(b.z1, -0.14630, 1.0e-5);
        assert_approx_eq!(b.z2, -0.10926, 1.0e-5);
        assert_approx_eq!(b.z3, 0.12963, 1.0e-5);
        assert_approx_eq!(b.w0, 0.17778, 1.0e-5);
        assert_approx_eq!(b.w1, 0.06667, 1.0e-5);
        assert_approx_eq!(b.w2, -0.26667, 1.0e-5);
        assert_approx_eq!(b.w3, 0.33333, 1.0e-5);
    }

    #[test]
    fn multiplying_a_matrix_product_by_its_inverse() {
        let a = matrix4(
            3., -9., 7., 3., 3., -8., 2., -9., -4., 4., 4., 1., -6., 5., -1., 1.,
        );
        let b = matrix4(
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        );
        let c = a * b;
        let d = c * b.inverse();

        assert_approx_eq!(d.x0, a.x0, 1.0e-5);
        assert_approx_eq!(d.x1, a.x1, 1.0e-5);
        assert_approx_eq!(d.x2, a.x2, 1.0e-5);
        assert_approx_eq!(d.x3, a.x3, 1.0e-5);
        assert_approx_eq!(d.y0, a.y0, 1.0e-5);
        assert_approx_eq!(d.y1, a.y1, 1.0e-5);
        assert_approx_eq!(d.y2, a.y2, 1.0e-5);
        assert_approx_eq!(d.y3, a.y3, 1.0e-5);
        assert_approx_eq!(d.z0, a.z0, 1.0e-5);
        assert_approx_eq!(d.z1, a.z1, 1.0e-5);
        assert_approx_eq!(d.z2, a.z2, 1.0e-5);
        assert_approx_eq!(d.z3, a.z3, 1.0e-5);
        assert_approx_eq!(d.w0, a.w0, 1.0e-5);
        assert_approx_eq!(d.w1, a.w1, 1.0e-5);
        assert_approx_eq!(d.w2, a.w2, 1.0e-5);
        assert_approx_eq!(d.w3, a.w3, 1.0e-5);
    }

    #[bench]
    fn bench_matrix_multiply(bencher: &mut Bencher) {
        let a = matrix4(
            3., -9., 7., 3., 3., -8., 2., -9., -4., 4., 4., 1., -6., 5., -1., 1.,
        );
        let b = matrix4(
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        );
        bencher.iter(|| a * b);
    }

    #[bench]
    fn bench_matrix_tuple_multiply(bencher: &mut Bencher) {
        let a = matrix4(
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        );
        let b = tuple4(1., 2., 3., 1.);
        bencher.iter(|| a * b);
    }

    #[bench]
    fn bench_matrix_inverse(bencher: &mut Bencher) {
        let m = matrix4(
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        );
        bencher.iter(|| m.inverse());
    }

    #[bench]
    fn bench_matrix_determinant(bencher: &mut Bencher) {
        let m = matrix4(
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        );
        bencher.iter(|| m.determinant());
    }

    #[bench]
    fn bench_matrix_cofactor(bencher: &mut Bencher) {
        let m = matrix4(
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        );
        bencher.iter(|| m.cofactor(1, 1));
    }

    #[bench]
    fn bench_matrix3_determinant(bencher: &mut Bencher) {
        let m = matrix3(1., 2., 6., -5., 8., -4., 2., 6., 4.);
        bencher.iter(|| m.determinant());
    }

    #[bench]
    fn bench_matrix3_cofactor(bencher: &mut Bencher) {
        let m = matrix3(1., 2., 6., -5., 8., -4., 2., 6., 4.);
        bencher.iter(|| m.cofactor(1, 1));
    }
}
