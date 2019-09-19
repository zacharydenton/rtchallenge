extern crate rtchallenge;
use rtchallenge::camera::*;
use rtchallenge::color::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::ppm::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;
use rtchallenge::world::*;

fn main() {
    let mut camera = camera(1000, 500, std::f32::consts::FRAC_PI_3);
    camera.set_transform(view_transform(
        point3(0., 7.0, 0.),
        point3(0., 0., 0.),
        vector3(0., 0., 1.),
    ));

    let mut world = world();
    world
        .lights
        .push(point_light(point3(0.8, 3.1, 0.7), color(1., 1., 1.)));

    let mut floor = plane();
    let mut floor_material = material();
    floor_material.color = color(1., 0.9, 0.9);
    floor_material.specular = 0.;
    floor.material = floor_material;
    world.objects.push(floor);

    let mut back_wall = plane();
    back_wall.transform = translate(0., 0., 1.5) * rotate_x(std::f32::consts::FRAC_PI_2);
    back_wall.material = floor_material;
    world.objects.push(back_wall);

    let mut front_wall = plane();
    front_wall.transform = translate(0., 0., -1.5) * rotate_x(-std::f32::consts::FRAC_PI_2);
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
    middle.transform = scale(0.8, 0.8, 0.8) * translate(-0.5, 1., 0.5);
    middle.material = material();
    middle.material.color = color(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    world.objects.push(middle);

    let mut right = sphere();
    right.transform = translate(1.2, 0.5, 0.) * scale(0.5, 0.5, 0.5);
    right.material = material();
    right.material.color = color(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;
    world.objects.push(right);

    let mut left = sphere();
    left.transform = scale(1., 2.5, 1.) * translate(-1.5, 0.33, -0.15) * scale(0.33, 0.33, 0.33);
    left.material = material();
    left.material.color = color(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;
    world.objects.push(left);

    let canvas = camera.render(&world);
    print!("{}", canvas_to_ppm(canvas));
}
