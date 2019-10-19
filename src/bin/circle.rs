extern crate rtchallenge;
use rtchallenge::canvas::*;
use rtchallenge::color::*;
use rtchallenge::geometry::*;
use rtchallenge::ppm::*;
use rtchallenge::ray::*;
use rtchallenge::tuple::*;

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let color = Color::new(1.0, 0.0, 0.0);
    let origin = point3(0.0, 0.0, -100.0);
    let sphere = Geometry::sphere();

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let u = (x as f32 - canvas.width as f32 / 2.) / (canvas.width as f32 / 2.);
            let v = -(y as f32 - canvas.width as f32 / 2.) / (canvas.width as f32 / 2.);
            let target = point3(u, v, 0.);
            let direction = target - origin;

            match sphere.intersect(ray(origin, direction)).next() {
                Some(_) => {
                    let x = (canvas.width as f32 / 2. + target.x * canvas.width as f32 / 2.).round()
                        as usize;
                    let y = (canvas.height as f32 / 2. - target.y * canvas.height as f32 / 2.)
                        .round() as usize;
                    canvas.set_color(x, y, color);
                }
                None => {}
            }
        }
    }

    print!("{}", canvas_to_ppm(canvas));
}
