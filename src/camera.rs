use crate::canvas::*;
use crate::ray::*;
use crate::scene::*;
use crate::transform::*;
use crate::tuple::*;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f32,
    transform: Transform,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

impl Camera {
    /// Constructs a camera with the given horizontal size (in pixels), vertical
    /// size (in pixels), and field of view (in radians).
    pub fn new(hsize: usize, vsize: usize, fov: f32) -> Self {
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
            transform: Transform::new(),
            half_width,
            half_height,
            pixel_size,
        }
    }

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
        let pixel = self.transform.world_to_local * point3(world_x, world_y, -1.);
        let origin = self.transform.world_to_local * point3(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        ray(origin, direction)
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn render(&self, scene: Scene) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..image.height {
            for x in 0..image.width {
                let ray = self.ray(x, y);
                let color = scene.color_at(ray);
                image.set_color(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::*;
    use crate::geometry::*;
    use crate::light::*;
    use crate::material::*;
    use crate::object::*;
    use assert_approx_eq::assert_approx_eq;
    use test::Bencher;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = std::f32::consts::FRAC_PI_2;
        let c = Camera::new(hsize, vsize, fov);

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.fov, fov);
        assert_eq!(c.transform, Transform::new());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f32::consts::FRAC_PI_2);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, std::f32::consts::FRAC_PI_2);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
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
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
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
        let mut c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        c.set_transform(
            Transform::new()
                .rotate_y(std::f32::consts::FRAC_PI_4)
                .translate(0., -2., 5.),
        );
        let r = c.ray(100, 50);

        assert_approx_eq!(r.origin.x, 0., 1e-5);
        assert_approx_eq!(r.origin.y, 2., 1e-5);
        assert_approx_eq!(r.origin.z, -5., 1e-5);
        assert_approx_eq!(r.direction.x, std::f32::consts::SQRT_2 / 2., 1e-5);
        assert_approx_eq!(r.direction.y, 0.0, 1e-5);
        assert_approx_eq!(r.direction.z, -std::f32::consts::SQRT_2 / 2., 1e-5);
    }

    #[test]
    fn rendering_a_scene_with_a_camera() {
        let mut scene = Scene::new();
        scene.add_light(Light::new(point3(-10., 10., -10.), Color::new(1., 1., 1.)));
        scene.add_object(
            Object::new().geometry(Geometry::sphere()).material(
                Material::new()
                    .color(Color::new(0.8, 1.0, 0.6))
                    .diffuse(0.7)
                    .specular(0.2),
            ),
        );
        scene.add_object(
            Object::new()
                .geometry(Geometry::sphere())
                .transform(Transform::new().scale(0.5, 0.5, 0.5)),
        );

        let mut camera = Camera::new(11, 11, std::f32::consts::FRAC_PI_2);
        let from = point3(0., 0., -5.);
        let to = point3(0., 0., 0.);
        let up = vector3(0., 1., 0.);
        camera.set_transform(Transform::look_at(from, to, up));

        let image = camera.render(scene);
        let pixel = image.get_color(5, 5);

        assert_approx_eq!(pixel.r, 0.38066, 1e-2);
        assert_approx_eq!(pixel.g, 0.47583, 1e-2);
        assert_approx_eq!(pixel.b, 0.2855, 1e-2);
    }

    #[bench]
    fn bench_constructing_a_ray_when_the_camera_is_transformed(bencher: &mut Bencher) {
        let mut c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        c.set_transform(
            Transform::new()
                .rotate_y(std::f32::consts::FRAC_PI_4)
                .translate(0., -2., 5.),
        );
        bencher.iter(|| c.ray(100, 50));
    }
}
