extern crate rtchallenge;
use rtchallenge::canvas::*;
use rtchallenge::color::*;
use rtchallenge::ppm::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let color = Color::new(1.0, 1.0, 1.0);
    let transform = Transform::new().rotate_z(std::f32::consts::PI / 6.);
    let mut point = point3(0.0, 0.8, 0.0);

    for _ in 0..12 {
        point = transform.local_to_world * point;
        let x = (canvas.width as f32 / 2. + point.x * canvas.width as f32 / 2.).round() as usize;
        let y = (canvas.height as f32 / 2. - point.y * canvas.height as f32 / 2.).round() as usize;
        canvas.set_color(x - 1, y, color);
        canvas.set_color(x, y, color);
        canvas.set_color(x + 1, y, color);
        canvas.set_color(x, y - 1, color);
        canvas.set_color(x, y, color);
        canvas.set_color(x, y + 1, color);
    }

    print!("{}", canvas_to_ppm(canvas));
}
