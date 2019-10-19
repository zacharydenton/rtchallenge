extern crate rtchallenge;
use rtchallenge::canvas::*;
use rtchallenge::color::*;
use rtchallenge::ppm::*;
use rtchallenge::tuple::*;

struct Projectile {
    position: Tuple4,
    velocity: Tuple4,
}

struct Environment {
    gravity: Tuple4,
    wind: Tuple4,
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut projectile = Projectile {
        position: point3(0., 1., 0.),
        velocity: vector3(1., 1.8, 0.).normalize() * 11.25,
    };
    let environment = Environment {
        gravity: vector3(0., -0.1, 0.),
        wind: vector3(-0.01, 0., 0.0),
    };
    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(0.8, 0.3, 0.2);

    while projectile.position.y > 0. {
        let x = projectile.position.x.round() as usize;
        let y = canvas.height - (projectile.position.y.round() as usize);
        canvas.set_color(x, y, color);
        projectile = tick(&environment, &projectile);
    }

    print!("{}", canvas_to_ppm(canvas));
}
