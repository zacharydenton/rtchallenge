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
        point3(-4.8, 0.8, -4.8),
        point3(-2., -3., -2.),
        vector3(0., 1., 0.),
    ));

    let mut world = world();
    world
        .lights
        .push(point_light(point3(-5., 4.5, -2.), color(1.8, 1.8, 1.8)));

    let mut floor = cube();
    floor.transform = scale(10., 10., 10.);
    let mut floor_material = material();
    let mut floor_pattern = checkers_pattern(color(1., 1., 1.), color(0., 0., 0.));;
    floor_pattern.transform = scale(0.1, 0.1, 0.1);
    floor_material.pattern = Some(floor_pattern);
    floor_material.diffuse = 0.4;
    floor_material.specular = 0.8;
    floor_material.shininess = 30.;
    floor_material.reflective = 0.2;
    floor.material = floor_material;
    world.objects.push(floor);

    let mut walls = cube();
    walls.transform = scale(9., 20., 9.);
    let mut walls_material = material();
    let mut walls_pattern = ring_pattern(color(1., 0., 0.), color(0.7, 0.8, 0.9));
    walls_pattern.transform = rotate_z(std::f32::consts::FRAC_PI_6) * scale(0.01, 0.01, 0.01);
    walls_material.pattern = Some(walls_pattern);
    walls_material.diffuse = 0.3;
    walls_material.specular = 0.3;
    walls_material.shininess = 100.;
    walls_material.reflective = 0.1;
    walls.material = walls_material;
    world.objects.push(walls);

    let mut table = cube();
    let mut table_material = material();
    let mut table_pattern = stripe_pattern(color(0.8, 0.5, 0.1), color(0.75, 0.45, 0.08));
    table_pattern.transform = rotate_y(5.3) * scale(0.05, 0.05, 0.05);
    table_material.pattern = Some(table_pattern);
    table_material.diffuse = 0.3;
    table_material.specular = 0.3;
    table_material.shininess = 100.;
    table_material.reflective = 0.02;
    table.material = table_material;
    table.transform = translate(0., -5.0, 0.) * scale(3., 0.3, 2.);
    world.objects.push(table);

    let mut leg = cube();
    leg.material = table_material;
    leg.transform = translate(-2.8, -7.5, -1.8)*  scale(0.2, 2.7, 0.2);
    world.objects.push(leg);

    let mut leg2 = cube();
    leg2.material = table_material;
    leg2.transform = translate(2.8, -7.5, -1.8)*  scale(0.2, 2.7, 0.2);
    world.objects.push(leg2);

    let mut leg3 = cube();
    leg3.material = table_material;
    leg3.transform = translate(-2.8, -7.5, 1.8)*  scale(0.2, 2.7, 0.2);
    world.objects.push(leg3);

    let mut leg4 = cube();
    leg4.material = table_material;
    leg4.transform = translate(2.8, -7.5, 1.8)*  scale(0.2, 2.7, 0.2);
    world.objects.push(leg4);

    let mut container = cube();
    container.material = material();
    container.material.color = color(0.5, 1., 0.1);
    container.material.diffuse = 0.7;
    container.material.specular = 0.3;
    container.material.reflective = 0.1;
    container.transform = translate(0.5, -4.5, 0.) * rotate_y(3.) * scale(0.3, 0.5, 0.3);
    world.objects.push(container);

    let canvas = camera.render(&world);
    print!("{}", canvas_to_ppm(canvas));
}

