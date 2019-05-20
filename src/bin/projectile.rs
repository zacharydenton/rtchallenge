extern crate rtchallenge;
use rtchallenge::canvas::*;
use rtchallenge::color::*;
use rtchallenge::ppm::*;
use rtchallenge::tuple::*;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut projectile = Projectile {
        position: point(0., 1., 0.),
        velocity: vector(1., 1.8, 0.).normalize() * 11.25,
    };
    let environment = Environment {
        gravity: vector(0., -0.1, 0.),
        wind: vector(-0.01, 0., 0.0),
    };
    let mut canvas = canvas(900, 550);
    let color = color(0.8, 0.3, 0.2);

    while projectile.position.y > 0. {
        let x = projectile.position.x.round() as usize;
        let y = canvas.height - (projectile.position.y.round() as usize);
        canvas.set_color(x, y, &color);
        projectile = tick(&environment, &projectile);
    }

    print!("{}", canvas_to_ppm(canvas));
}
