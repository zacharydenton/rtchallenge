extern crate rtchallenge;
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
        velocity: vector(1., 1., 0.).normalize(),
    };
    let environment = Environment {
        gravity: vector(0., -0.1, 0.),
        wind: vector(-0.01, 0., 0.0),
    };

    println!(
        "Launching a projectile from {:?} with velocity {:?}.",
        projectile.position, projectile.velocity
    );
    println!(
        "The current gravity is {:?} and the windspeed is {:?}.",
        environment.gravity, environment.wind
    );

    let mut ticks = 0;
    while projectile.position.y > 0. {
        println!("Projectile position: {:?}", projectile.position);
        projectile = tick(&environment, &projectile);
        ticks += 1;
    }

    println!("The projectile crashed after {} ticks.", ticks);
}
