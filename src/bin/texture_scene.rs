extern crate rtchallenge;
use rtchallenge::camera::*;
use rtchallenge::color::*;
use rtchallenge::geometry::*;
use rtchallenge::light::*;
use rtchallenge::material::*;
use rtchallenge::object::*;
use rtchallenge::ppm::*;
use rtchallenge::scene::*;
use rtchallenge::texture::*;
use rtchallenge::transform::*;
use rtchallenge::tuple::*;

fn main() {
    let mut camera = Camera::new(1000, 500, std::f32::consts::FRAC_PI_3);
    camera.set_transform(Transform::look_at(
        point3(-0.2, 2.8, -1.4),
        point3(0.3, 0.0, 1.),
        vector3(0., 1., 0.),
    ));

    let mut scene = Scene::new();
    scene.add_light(Light::new(point3(-0.5, 2.7, -1.3), Color::new(1., 1., 1.)));

    let mut floor_texture = Texture::checkerboard_2d(Color::WHITE, Color::BLACK);
    floor_texture.transform = Transform::new().scale(0.2, 0.2, 0.2);
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .material(Material::new().texture(floor_texture).specular(0.1)),
    );

    let mut wall_texture = Texture::ring(Color::new(1., 1., 1.), Color::new(0.1, 0.1, 0.9));
    wall_texture.transform = Transform::new().scale(0.2, 0.2, 0.2);
    let wall_material = Material::new().texture(wall_texture).specular(0.2);
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .transform(
                Transform::new()
                    .translate(0., 0., 1.5)
                    .rotate_x(std::f32::consts::FRAC_PI_2),
            )
            .material(wall_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .transform(
                Transform::new()
                    .translate(0., 0., -1.5)
                    .rotate_x(-std::f32::consts::FRAC_PI_2),
            )
            .material(wall_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .transform(
                Transform::new()
                    .translate(0., 0., 2.2)
                    .rotate_y(-std::f32::consts::FRAC_PI_4)
                    .rotate_x(std::f32::consts::FRAC_PI_2),
            )
            .material(wall_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .transform(
                Transform::new()
                    .translate(0., 0., -2.2)
                    .rotate_y(std::f32::consts::FRAC_PI_4)
                    .rotate_x(-std::f32::consts::FRAC_PI_2),
            )
            .material(wall_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .transform(
                Transform::new()
                    .translate(0., 0., 2.2)
                    .rotate_y(std::f32::consts::FRAC_PI_4)
                    .rotate_x(std::f32::consts::FRAC_PI_2),
            )
            .material(wall_material),
    );
    scene.add_object(
        Object::new()
            .geometry(Geometry::plane())
            .transform(
                Transform::new()
                    .translate(0., 0., -2.2)
                    .rotate_y(-std::f32::consts::FRAC_PI_4)
                    .rotate_x(-std::f32::consts::FRAC_PI_2),
            )
            .material(wall_material),
    );

    let mut middle_texture =
        Texture::checkerboard_3d(Color::new(0.1, 1., 0.5), Color::new(1., 1., 1.));
    middle_texture.transform = Transform::new().scale(0.5, 0.5, 0.5);
    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .transform(
                Transform::new()
                    .scale(0.8, 0.8, 0.8)
                    .translate(-0.5, 1., 0.5)
                    .rotate_z(std::f32::consts::FRAC_PI_2),
            )
            .material(
                Material::new()
                    .texture(middle_texture)
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );

    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .transform(
                Transform::new()
                    .translate(1.2, 0.5, 0.)
                    .scale(0.5, 0.5, 0.5)
                    .rotate_y(2.)
                    .rotate_z(1.35),
            )
            .material(
                Material::new()
                    .texture(Texture::linear_gradient(
                        Color::new(1.0, 0.1, 0.2),
                        Color::new(0.1, 0.4, 0.9),
                    ))
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );

    let mut left_texture = Texture::white_noise();
    left_texture.transform = Transform::new()
        .scale(0.2, 1., 1.)
        .rotate_y(std::f32::consts::FRAC_PI_4);
    scene.add_object(
        Object::new()
            .geometry(Geometry::sphere())
            .transform(
                Transform::new()
                    .scale(1., 2.5, 1.)
                    .translate(0.4, 0.33, -0.45)
                    .scale(0.33, 0.33, 0.33),
            )
            .material(
                Material::new()
                    .texture(left_texture)
                    .diffuse(0.7)
                    .specular(0.3),
            ),
    );

    let canvas = camera.render(scene);
    print!("{}", canvas_to_ppm(canvas));
}
