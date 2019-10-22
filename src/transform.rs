use crate::matrix::*;
use crate::tuple::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform {
    pub local_to_world: Matrix4,
    pub world_to_local: Matrix4,
}

impl Transform {
    /// Creates an identity transform.
    pub fn new() -> Self {
        Transform {
            local_to_world: I4,
            world_to_local: I4,
        }
    }

    /// Creates a view transform given the point where the eye is looking
    /// from, the point where the eye is looking to, and a vector indicating
    /// which direction is up.
    pub fn look_at(from: Tuple4, to: Tuple4, up: Tuple4) -> Self {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);
        let orientation = matrix4(
            left.x, left.y, left.z, 0., true_up.x, true_up.y, true_up.z, 0., -forward.x,
            -forward.y, -forward.z, 0., 0., 0., 0., 1.,
        );

        // translate(-from.x, -from.y, -from.z)
        let view_matrix = orientation
            * matrix4(
                1., 0., 0., -from.x, 0., 1., 0., -from.y, 0., 0., 1., -from.z, 0., 0., 0., 1.,
            );

        Transform {
            local_to_world: view_matrix,
            world_to_local: view_matrix.inverse(),
        }
    }

    /// Translates by the specified amount in each axis.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> Self {
        let translate_matrix = matrix4(1., 0., 0., x, 0., 1., 0., y, 0., 0., 1., z, 0., 0., 0., 1.);
        self.local_to_world = self.local_to_world * translate_matrix;
        self.world_to_local = self.local_to_world.inverse();
        *self
    }

    /// Rotates around the x-axis by the angle in radians.
    pub fn rotate_x(&mut self, radians: f32) -> Self {
        let rotation_matrix = matrix4(
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
        );
        self.local_to_world = self.local_to_world * rotation_matrix;
        self.world_to_local = self.local_to_world.inverse();
        *self
    }

    /// Rotates around the y-axis by the angle in radians.
    pub fn rotate_y(&mut self, radians: f32) -> Self {
        let rotation_matrix = matrix4(
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
        );
        self.local_to_world = self.local_to_world * rotation_matrix;
        self.world_to_local = self.local_to_world.inverse();
        *self
    }

    /// Rotates around the z-axis by the angle in radians.
    pub fn rotate_z(&mut self, radians: f32) -> Self {
        let rotation_matrix = matrix4(
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
        );
        self.local_to_world = self.local_to_world * rotation_matrix;
        self.world_to_local = self.local_to_world.inverse();
        *self
    }

    /// Scales by the specified amount in each axis.
    pub fn scale(&mut self, x: f32, y: f32, z: f32) -> Self {
        let scale_matrix = matrix4(x, 0., 0., 0., 0., y, 0., 0., 0., 0., z, 0., 0., 0., 0., 1.);
        self.local_to_world = self.local_to_world * scale_matrix;
        self.world_to_local = self.local_to_world.inverse();
        *self
    }

    /// Applies the shear transformation.
    pub fn shear(&mut self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let shear_matrix = matrix4(
            1., xy, xz, 0., yx, 1., yz, 0., zx, zy, 1., 0., 0., 0., 0., 1.,
        );
        self.local_to_world = self.local_to_world * shear_matrix;
        self.world_to_local = self.local_to_world.inverse();
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Transform::new().translate(5., -3., 2.);
        let p = point3(-3., 4., 5.);
        assert_eq!(transform.local_to_world * p, point3(2., 1., 7.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transform::new().translate(5., -3., 2.);
        let p = point3(-3., 4., 5.);
        assert_eq!(transform.world_to_local * p, point3(-8., 7., 3.));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transform::new().translate(5., -3., 2.);
        let v = vector3(-3., 4., 5.);
        assert_eq!(transform.local_to_world * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let transform = Transform::new().scale(2., 3., 4.);
        let p = point3(-4., 6., 8.);
        assert_eq!(transform.local_to_world * p, point3(-8., 18., 32.));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let transform = Transform::new().scale(2., 3., 4.);
        let v = vector3(-4., 6., 8.);
        assert_eq!(transform.local_to_world * v, vector3(-8., 18., 32.));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transform::new().scale(2., 3., 4.);
        let v = vector3(-4., 6., 8.);
        assert_eq!(transform.world_to_local * v, vector3(-2., 2., 2.));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Transform::new().scale(-1., 1., 1.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform.local_to_world * p, point3(-2., 3., 4.));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = point3(0., 1., 0.);
        let half_quarter = Transform::new().rotate_x(std::f32::consts::FRAC_PI_4);
        let full_quarter = Transform::new().rotate_x(std::f32::consts::FRAC_PI_2);

        let half_rotation = half_quarter.local_to_world * p;
        assert_approx_eq!(half_rotation.x, 0.);
        assert_approx_eq!(half_rotation.y, std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.z, std::f32::consts::SQRT_2 / 2.);

        let full_rotation = full_quarter.local_to_world * p;
        assert_approx_eq!(full_rotation.x, 0.);
        assert_approx_eq!(full_rotation.y, 0.);
        assert_approx_eq!(full_rotation.z, 1.);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = point3(0., 0., 1.);
        let half_quarter = Transform::new().rotate_y(std::f32::consts::FRAC_PI_4);
        let full_quarter = Transform::new().rotate_y(std::f32::consts::FRAC_PI_2);

        let half_rotation = half_quarter.local_to_world * p;
        assert_approx_eq!(half_rotation.x, std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.y, 0.);
        assert_approx_eq!(half_rotation.z, std::f32::consts::SQRT_2 / 2.);

        let full_rotation = full_quarter.local_to_world * p;
        assert_approx_eq!(full_rotation.x, 1.);
        assert_approx_eq!(full_rotation.y, 0.);
        assert_approx_eq!(full_rotation.z, 0.);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = point3(0., 1., 0.);
        let half_quarter = Transform::new().rotate_z(std::f32::consts::FRAC_PI_4);
        let full_quarter = Transform::new().rotate_z(std::f32::consts::FRAC_PI_2);

        let half_rotation = half_quarter.local_to_world * p;
        assert_approx_eq!(half_rotation.x, -std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.y, std::f32::consts::SQRT_2 / 2.);
        assert_approx_eq!(half_rotation.z, 0.);

        let full_rotation = full_quarter.local_to_world * p;
        assert_approx_eq!(full_rotation.x, -1.);
        assert_approx_eq!(full_rotation.y, 0.);
        assert_approx_eq!(full_rotation.z, 0.);
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Transform::new().shear(1., 0., 0., 0., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform.local_to_world * p, point3(5., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Transform::new().shear(0., 1., 0., 0., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform.local_to_world * p, point3(6., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Transform::new().shear(0., 0., 1., 0., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform.local_to_world * p, point3(2., 5., 4.));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Transform::new().shear(0., 0., 0., 1., 0., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform.local_to_world * p, point3(2., 7., 4.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Transform::new().shear(0., 0., 0., 0., 1., 0.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform.local_to_world * p, point3(2., 3., 6.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Transform::new().shear(0., 0., 0., 0., 0., 1.);
        let p = point3(2., 3., 4.);
        assert_eq!(transform.local_to_world * p, point3(2., 3., 7.));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = point3(1., 0., 1.);
        let a = Transform::new().rotate_x(std::f32::consts::FRAC_PI_2);
        let b = Transform::new().scale(5., 5., 5.);
        let c = Transform::new().translate(10., 5., 7.);

        // Apply rotation first
        let p2 = a.local_to_world * p;
        assert_approx_eq!(p2.x, 1.);
        assert_approx_eq!(p2.y, -1.);
        assert_approx_eq!(p2.z, 0.);

        // Then apply scaling
        let p3 = b.local_to_world * p2;
        assert_approx_eq!(p3.x, 5.);
        assert_approx_eq!(p3.y, -5.);
        assert_approx_eq!(p3.z, 0.);

        // Then apply translation
        let p4 = c.local_to_world * p3;
        assert_approx_eq!(p4.x, 15.);
        assert_approx_eq!(p4.y, 0.);
        assert_approx_eq!(p4.z, 7.);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = point3(1., 0., 1.);
        let a = Transform::new().rotate_x(std::f32::consts::FRAC_PI_2);
        let b = Transform::new().scale(5., 5., 5.);
        let c = Transform::new().translate(10., 5., 7.);

        let t = c.local_to_world * b.local_to_world * a.local_to_world;
        let p2 = t * p;
        assert_approx_eq!(p2.x, 15.);
        assert_approx_eq!(p2.y, 0.);
        assert_approx_eq!(p2.z, 7.);
    }

    #[test]
    fn chained_transformation_methods_must_be_applied_in_trs_order() {
        let p = point3(1., 0., 1.);
        let t = Transform::new()
            .translate(10., 5., 7.)
            .rotate_x(std::f32::consts::FRAC_PI_2)
            .scale(5., 5., 5.);

        let p2 = t.local_to_world * p;
        assert_approx_eq!(p2.x, 15.);
        assert_approx_eq!(p2.y, 0.);
        assert_approx_eq!(p2.z, 7.);
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = point3(0., 0., 0.);
        let to = point3(0., 0., -1.);
        let up = vector3(0., 1., 0.);
        let t = Transform::look_at(from, to, up);
        assert_eq!(t.local_to_world, I4);
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point3(0., 0., 0.);
        let to = point3(0., 0., 1.);
        let up = vector3(0., 1., 0.);
        let t = Transform::look_at(from, to, up);
        assert_eq!(t, Transform::new().scale(-1., 1., -1.));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = point3(0., 0., 8.);
        let to = point3(0., 0., 0.);
        let up = vector3(0., 1., 0.);
        let t = Transform::look_at(from, to, up);
        assert_eq!(t, Transform::new().translate(0., 0., -8.));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = point3(1., 3., 2.);
        let to = point3(4., -2., 8.);
        let up = vector3(1., 1., 0.);
        let t = Transform::look_at(from, to, up).local_to_world;

        assert_approx_eq!(t.x0, -0.50709, 1e-5);
        assert_approx_eq!(t.x1, 0.50709, 1e-5);
        assert_approx_eq!(t.x2, 0.67612, 1e-5);
        assert_approx_eq!(t.x3, -2.36643, 1e-5);
        assert_approx_eq!(t.y0, 0.76772, 1e-5);
        assert_approx_eq!(t.y1, 0.60609, 1e-5);
        assert_approx_eq!(t.y2, 0.12122, 1e-5);
        assert_approx_eq!(t.y3, -2.82843, 1e-5);
        assert_approx_eq!(t.z0, -0.35857, 1e-5);
        assert_approx_eq!(t.z1, 0.59761, 1e-5);
        assert_approx_eq!(t.z2, -0.71714, 1e-5);
        assert_approx_eq!(t.z3, 0.00000, 1e-5);
        assert_approx_eq!(t.w0, 0.00000, 1e-5);
        assert_approx_eq!(t.w1, 0.00000, 1e-5);
        assert_approx_eq!(t.w2, 0.00000, 1e-5);
        assert_approx_eq!(t.w3, 1.00000, 1e-5);
    }
}
