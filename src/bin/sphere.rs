extern crate rtchallenge;
use rtchallenge::canvas::*;
use rtchallenge::color::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::ppm::*;
use rtchallenge::ray::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;

fn main() {
    let mut canvas = canvas(1000, 1000);
    let origin = point3(0.0, 0.0, -1.0);
    let mut sphere = sphere();
    sphere.transform = scale(0.5, 0.5, 0.5);
    sphere.material = material();
    sphere.material.color = color(0.01, 0.01, 0.2);

    let light = point_light(point3(-10., 10., -10.), color(1., 1., 1.));

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let u = (x as f32 - canvas.width as f32 / 2.) / (canvas.width as f32 / 2.);
            let v = -(y as f32 - canvas.width as f32 / 2.) / (canvas.width as f32 / 2.);
            let target = point3(u, v, 0.);
            let direction = (target - origin).normalize();
            let ray = ray(origin, direction);

            match hit(sphere.intersect(&ray)) {
                Some(Intersection { t, object }) => {
                    let point = ray.position(t);
                    let normal = object.normal(point);
                    let eye = -ray.direction;
                    let color = object.material.lighting(&light, &point, &eye, &normal);

                    let x = (canvas.width as f32 / 2. + target.x * canvas.width as f32 / 2.).round()
                        as usize;
                    let y = (canvas.height as f32 / 2. - target.y * canvas.height as f32 / 2.)
                        .round() as usize;
                    canvas.set_color(x, y, &color);
                }
                None => {}
            }
        }
    }

    print!("{}", canvas_to_ppm(canvas));
}
