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
        point3(-0.2, 2.8, -1.4),
        point3(0.3, 0.0, 1.),
        vector3(0., 1., 0.),
    ));

    let mut world = world();
    world
        .lights
        .push(point_light(point3(-0.5, 2.7, -1.3), color(1., 1., 1.)));

    let mut floor = plane();
    let mut floor_material = material();
    let mut floor_pattern = ring_pattern(color(1., 1., 1.), color(0.1, 0.1, 0.9));
    floor_pattern.transform = scale(0.2, 0.2, 0.2);
    floor_material.pattern = Some(floor_pattern);
    floor_material.specular = 0.;
    floor.material = floor_material;
    world.objects.push(floor);

    let mut back_wall = plane();
    back_wall.transform = translate(0., 0., 1.5)
        * rotate_x(std::f32::consts::FRAC_PI_2);
    back_wall.material = floor_material;
    world.objects.push(back_wall);

    let mut front_wall = plane();
    front_wall.transform = translate(0., 0., -1.5)
        * rotate_x(-std::f32::consts::FRAC_PI_2);
    front_wall.material = floor_material;
    world.objects.push(front_wall);

    let mut left_wall = plane();
    left_wall.transform = translate(0., 0., 2.2)
        * rotate_y(-std::f32::consts::FRAC_PI_4)
        * rotate_x(std::f32::consts::FRAC_PI_2);
    left_wall.material = floor_material;
    world.objects.push(left_wall);

    let mut left_wall2 = plane();
    left_wall2.transform = translate(0., 0., -2.2)
        * rotate_y(std::f32::consts::FRAC_PI_4)
        * rotate_x(-std::f32::consts::FRAC_PI_2);
    left_wall2.material = floor_material;
    world.objects.push(left_wall2);

    let mut right_wall = plane();
    right_wall.transform = translate(0., 0., 2.2)
        * rotate_y(std::f32::consts::FRAC_PI_4)
        * rotate_x(std::f32::consts::FRAC_PI_2);
    right_wall.material = floor_material;
    world.objects.push(right_wall);

    let mut right_wall2 = plane();
    right_wall2.transform = translate(0., 0., -2.2)
        * rotate_y(-std::f32::consts::FRAC_PI_4)
        * rotate_x(-std::f32::consts::FRAC_PI_2);
    right_wall2.material = floor_material;
    world.objects.push(right_wall2);

    let mut middle = sphere();
    middle.transform = scale(0.8, 0.8, 0.8) * translate(-0.5, 1., 0.5) * rotate_z(std::f32::consts::FRAC_PI_2);
    middle.material = material();
    let mut middle_pattern = checkers_pattern(color(0.1, 1., 0.5), color(1., 1., 1.));
    middle_pattern.transform = scale(0.5, 0.5, 0.5);
    middle.material.pattern = Some(middle_pattern);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.objects.push(middle);

    let mut right = sphere();
    right.transform =
        translate(1.2, 0.5, 0.) * scale(0.5, 0.5, 0.5) * rotate_y(2.) * rotate_z(1.35);
    right.material = material();
    let right_pattern = gradient_pattern(color(1.0, 0.1, 0.2), color(0.1, 0.4, 0.9));
    right.material.pattern = Some(right_pattern);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.objects.push(right);

    let mut left = sphere();
    left.transform = scale(1., 2.5, 1.) * translate(0.4, 0.33, -0.45) * scale(0.33, 0.33, 0.33);
    left.material = material();
    let mut left_pattern = stripe_pattern(color(1., 0., 0.), color(0.7, 0.8, 0.9));
    left_pattern.transform = scale(0.2, 1., 1.) * rotate_y(std::f32::consts::FRAC_PI_4);
    left.material.pattern = Some(left_pattern);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.objects.push(left);

    let canvas = camera.render(&world);
    print!("{}", canvas_to_ppm(canvas));
}
