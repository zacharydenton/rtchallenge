use crate::matrix::*;
use crate::ray::*;
use crate::tuple::*;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f32,
    transform: Matrix4,
    inverse_transform: Matrix4,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

/// Constructs a camera with the given horizontal size (in pixels), vertical
/// size (in pixels), and field of view (in radians).
pub fn camera(hsize: usize, vsize: usize, fov: f32) -> Camera {
    let half_view = (fov / 2.).tan();
    let aspect = hsize as f32 / vsize as f32;

    let (half_width, half_height) = if aspect >= 1. {
        (half_view, half_view / aspect)
    } else {
        (half_view * aspect, half_view)
    };

    let pixel_size = (half_width * 2.) / hsize as f32;

    Camera {
        hsize,
        vsize,
        fov,
        transform: I4,
        inverse_transform: I4,
        half_width,
        half_height,
        pixel_size,
    }
}

impl Camera {
    /// Returns a ray that starts at the camera and passes through the indicated
    /// (x, y) pixel on the canvas.
    pub fn ray(&self, x: usize, y: usize) -> Ray {
        // The offset from the edge of the canvas to the pixel's center.
        let xoffset = (x as f32 + 0.5) * self.pixel_size;
        let yoffset = (y as f32 + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space.
        // (The camera looks toward -z, so +x is to the left.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // Using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (The canvas is at z = -1.)
        let pixel = self.inverse_transform * point3(world_x, world_y, -1.);
        let origin = self.inverse_transform * point3(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        ray(origin, direction)
    }

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::*;
    use assert_approx_eq::assert_approx_eq;
    use test::Bencher;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = std::f32::consts::FRAC_PI_2;
        let c = camera(hsize, vsize, fov);

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.fov, fov);
        assert_eq!(c.transform, I4);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = camera(200, 125, std::f32::consts::FRAC_PI_2);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = camera(125, 200, std::f32::consts::FRAC_PI_2);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = camera(201, 101, std::f32::consts::FRAC_PI_2);
        let r = c.ray(100, 50);

        assert_approx_eq!(r.origin.x, 0., 1e-5);
        assert_approx_eq!(r.origin.y, 0., 1e-5);
        assert_approx_eq!(r.origin.z, 0., 1e-5);
        assert_approx_eq!(r.direction.x, 0., 1e-5);
        assert_approx_eq!(r.direction.y, 0., 1e-5);
        assert_approx_eq!(r.direction.z, -1., 1e-5);
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = camera(201, 101, std::f32::consts::FRAC_PI_2);
        let r = c.ray(0, 0);

        assert_approx_eq!(r.origin.x, 0., 1e-5);
        assert_approx_eq!(r.origin.y, 0., 1e-5);
        assert_approx_eq!(r.origin.z, 0., 1e-5);
        assert_approx_eq!(r.direction.x, 0.66519, 1e-5);
        assert_approx_eq!(r.direction.y, 0.33259, 1e-5);
        assert_approx_eq!(r.direction.z, -0.66851, 1e-5);
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = camera(201, 101, std::f32::consts::FRAC_PI_2);
        c.set_transform(rotate_y(std::f32::consts::FRAC_PI_4) * translate(0., -2., 5.));
        let r = c.ray(100, 50);

        assert_approx_eq!(r.origin.x, 0., 1e-5);
        assert_approx_eq!(r.origin.y, 2., 1e-5);
        assert_approx_eq!(r.origin.z, -5., 1e-5);
        assert_approx_eq!(r.direction.x, std::f32::consts::SQRT_2 / 2., 1e-5);
        assert_approx_eq!(r.direction.y, 0.0, 1e-5);
        assert_approx_eq!(r.direction.z, -std::f32::consts::SQRT_2 / 2., 1e-5);
    }

    #[bench]
    fn bench_constructing_a_ray_when_the_camera_is_transformed(bencher: &mut Bencher) {
        let mut c = camera(201, 101, std::f32::consts::FRAC_PI_2);
        c.set_transform(rotate_y(std::f32::consts::FRAC_PI_4) * translate(0., -2., 5.));
        bencher.iter(|| c.ray(100, 50));
    }
}
