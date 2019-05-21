extern crate rtchallenge;
use rtchallenge::canvas::*;
use rtchallenge::color::*;
use rtchallenge::object::*;
use rtchallenge::ppm::*;
use rtchallenge::ray::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;

fn main() {
    let mut canvas = canvas(1000, 1000);
    let color = color(1.0, 0.0, 0.0);
    let origin = point3(0.0, 0.0, -1.0);
    let mut sphere = sphere();
    sphere.transform = rotate_z(0.25) * shear(1., 0., 0., 0., 0., 0.) * scale(0.5, 0.5, 0.5);

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let u = (x as f32 - canvas.width as f32 / 2.) / (canvas.width as f32 / 2.);
            let v = -(y as f32 - canvas.width as f32 / 2.) / (canvas.width as f32 / 2.);
            let target = point3(u, v, 0.);
            let direction = target - origin;

            match hit(sphere.intersect(ray(origin, direction))) {
                Some(_) => {
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
