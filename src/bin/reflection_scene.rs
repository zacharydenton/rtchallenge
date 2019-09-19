extern crate rtchallenge;
use rtchallenge::camera::*;
use rtchallenge::color::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::pattern::*;
use rtchallenge::ppm::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;
use rtchallenge::world::*;

fn main() {
    let mut camera = camera(1000, 500, std::f32::consts::FRAC_PI_3);
    camera.set_transform(view_transform(
        point3(0., 1.5, -5.),
        point3(0., 1., 0.),
        vector3(0., 1., 0.),
    ));

    let mut world = world();
    world
        .lights
        .push(point_light(point3(-10., 10., -10.), color(1., 1., 1.)));

    let mut floor = sphere();
    floor.transform = scale(10., 0.01, 10.);
    let mut floor_material = material();
    let mut floor_pattern = checkers_pattern(color(1., 1., 1.), color(0., 0., 0.));;
    floor_pattern.transform = scale(0.05, 0.05, 0.05);
    floor_material.pattern = Some(floor_pattern);
    floor_material.specular = 0.;
    floor.material = floor_material;
    world.objects.push(floor);

    let mut left_wall = sphere();
    left_wall.transform = translate(0., 0., 5.)
        * rotate_y(-std::f32::consts::FRAC_PI_4)
        * rotate_x(std::f32::consts::FRAC_PI_2)
        * scale(10., 0.01, 10.);
    left_wall.material = floor_material;
    world.objects.push(left_wall);

    let mut right_wall = sphere();
    right_wall.transform = translate(0., 0., 5.)
        * rotate_y(std::f32::consts::FRAC_PI_4)
        * rotate_x(std::f32::consts::FRAC_PI_2)
        * scale(10., 0.01, 10.);
    right_wall.material = floor_material;
    world.objects.push(right_wall);

    let mut middle = sphere();
    middle.transform = translate(-0.5, 1., 0.5);
    middle.material = material();
    middle.material.color = color(0.5, 0.5, 0.5);
    middle.material.diffuse = 0.3;
    middle.material.specular = 0.15;
    middle.material.shininess = 1000.;
    middle.material.reflective = 0.7;
    world.objects.push(middle);

    let mut right = sphere();
    right.transform = translate(1.2, 1.5, 0.) * scale(0.5, 0.5, 0.5);
    right.material = material();
    right.material.color = color(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    right.material.reflective = 0.1;
    world.objects.push(right);

    let mut left = sphere();
    left.transform = translate(-0.9, 0.33, -0.75) * scale(0.25, 0.25, 0.25);
    left.material = material();
    left.material.color = color(1., 0.2, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    left.material.reflective = 0.1;
    world.objects.push(left);

    let mut left2 = sphere();
    left2.transform = translate(-0.3, 1.73, -0.95) * scale(0.35, 0.35, 0.35);
    left2.material = material();
    left2.material.color = color(0.2, 0.1, 1.0);
    left2.material.diffuse = 0.7;
    left2.material.specular = 0.3;
    left2.material.reflective = 0.2;
    world.objects.push(left2);

    let canvas = camera.render(&world);
    print!("{}", canvas_to_ppm(canvas));
}
