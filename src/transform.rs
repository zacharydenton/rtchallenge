use crate::matrix::*;

pub fn translate(x: f32, y: f32, z: f32) -> Matrix4 {
    matrix4(1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.)
}

pub fn scale(x: f32, y: f32, z: f32) -> Matrix4 {
    matrix4(x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 1.)
}

pub fn rotate_x(radians: f32) -> Matrix4 {
    matrix4(
        1.,
        0.,
        0.,
        0.,
        0.,
        radians.cos(),
        -radians.sin(),
        0.,
        0.,
        radians.sin(),
        radians.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    )
}

pub fn rotate_y(radians: f32) -> Matrix4 {
    matrix4(
        radians.cos(),
        0.,
        radians.sin(),
        0.,
        0.,
        1.,
        0.,
        0.,
        -radians.sin(),
        0.,
        radians.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    )
}

pub fn rotate_z(radians: f32) -> Matrix4 {
    matrix4(
        radians.cos(),
        -radians.sin(),
        0.,
        0.,
        radians.sin(),
        radians.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        0.,
        0.,
        0.,
        1.,
    )
}

pub fn shear(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix4 {
    matrix4(
        1., xy, xz, 0., yx, 1., yz, 0., zx, zy, 1., 0., 0., 0., 0., 1.,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translate(5., -3., 2.);
        let p = point3(-3., 4., 5.);
        assert_eq!(transform * p, point3(2., 1., 7.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translate(5., -3., 2.);
        let inverse = transform.inverse();
        let p = point3(-3., 4., 5.);
        assert_eq!(inverse * p, point3(-8., 7., 3.));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translate(5., -3., 2.);
        let v = vector3(-3., 4., 5.);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = scale(2., 3., 4.);
        let p = point3(-4., 6., 8.);
        assert_eq!(transform * p, point3(-8., 18., 32.));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = scale(2., 3., 4.);
        let v = vector3(-4., 6., 8.);
        assert_eq!(transform * v, vector3(-8., 18., 32.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scale(2., 3., 4.);
        let inverse = transform.inverse();
        let v = vector3(-4., 6., 8.);
        assert_eq!(inverse * v, vector3(-2., 2., 2.));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = scale(-1., 1., 1.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform * p, point3(-2., 3., 4.));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = point3(0., 1., 0.);
        let half_quarter = rotate_x(std::f32::consts::FRAC_PI_4);
        let full_quarter = rotate_x(std::f32::consts::FRAC_PI_2);

        let half_rotation = half_quarter * p;
        assert_approx_eq!(half_rotation.x, 0.);
        assert_approx_eq!(half_rotation.y, std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.z, std::f32::consts::SQRT_2 / 2.);

        let full_rotation = full_quarter * p;
        assert_approx_eq!(full_rotation.x, 0.);
        assert_approx_eq!(full_rotation.y, 0.);
        assert_approx_eq!(full_rotation.z, 1.);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = point3(0., 0., 1.);
        let half_quarter = rotate_y(std::f32::consts::FRAC_PI_4);
        let full_quarter = rotate_y(std::f32::consts::FRAC_PI_2);

        let half_rotation = half_quarter * p;
        assert_approx_eq!(half_rotation.x, std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.y, 0.);
        assert_approx_eq!(half_rotation.z, std::f32::consts::SQRT_2 / 2.);

        let full_rotation = full_quarter * p;
        assert_approx_eq!(full_rotation.x, 1.);
        assert_approx_eq!(full_rotation.y, 0.);
        assert_approx_eq!(full_rotation.z, 0.);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = point3(0., 1., 0.);
        let half_quarter = rotate_z(std::f32::consts::FRAC_PI_4);
        let full_quarter = rotate_z(std::f32::consts::FRAC_PI_2);

        let half_rotation = half_quarter * p;
        assert_approx_eq!(half_rotation.x, -std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.y, std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.z, 0.);

        let full_rotation = full_quarter * p;
        assert_approx_eq!(full_rotation.x, -1.);
        assert_approx_eq!(full_rotation.y, 0.);
        assert_approx_eq!(full_rotation.z, 0.);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shear(1., 0., 0., 0., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform * p, point3(5., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shear(0., 1., 0., 0., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform * p, point3(6., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shear(0., 0., 1., 0., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform * p, point3(2., 5., 4.));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shear(0., 0., 0., 1., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform * p, point3(2., 7., 4.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shear(0., 0., 0., 0., 1., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform * p, point3(2., 3., 6.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shear(0., 0., 0., 0., 0., 1.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform * p, point3(2., 3., 7.));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = point3(1., 0., 1.);
        let a = rotate_x(std::f32::consts::FRAC_PI_2);
        let b = scale(5., 5., 5.);
        let c = translate(10., 5., 7.);

        // Apply rotation first
        let p2 = a * p;
        assert_approx_eq!(p2.x, 1.);
        assert_approx_eq!(p2.y, -1.);
        assert_approx_eq!(p2.z, 0.);

        // Then apply scaling
        let p3 = b * p2;
        assert_approx_eq!(p3.x, 5.);
        assert_approx_eq!(p3.y, -5.);
        assert_approx_eq!(p3.z, 0.);

        // Then apply translation
        let p4 = c * p3;
        assert_approx_eq!(p4.x, 15.);
        assert_approx_eq!(p4.y, 0.);
        assert_approx_eq!(p4.z, 7.);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = point3(1., 0., 1.);
        let a = rotate_x(std::f32::consts::FRAC_PI_2);
        let b = scale(5., 5., 5.);
        let c = translate(10., 5., 7.);

        let t = c * b * a;
        let p2 = t * p;
        assert_approx_eq!(p2.x, 15.);
        assert_approx_eq!(p2.y, 0.);
        assert_approx_eq!(p2.z, 7.);
    }
}
