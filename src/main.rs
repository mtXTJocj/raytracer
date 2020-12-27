use raytracer::{
    camera::Camera, color::Color, light::Light, plane::Plane, point3d::Point3D,
    shape::Shape, sphere::Sphere, transform::Transform, vector3d::Vector3D,
    world::World, FLOAT,
};

use std::{
    boxed::Box,
    env,
    fs::File,
    io::{stdout, BufWriter, Write},
};

fn main() {
    let mut writer: Box<dyn Write> = match env::args().nth(1) {
        None => Box::new(BufWriter::new(stdout())),
        Some(filename) => Box::new(BufWriter::new(
            File::create(filename).expect("cannot create file"),
        )),
    };

    let mut world = World::new();

    let mut floor = Box::new(Plane::new());
    floor.material_mut().color = Color::new(1.0, 0.9, 0.9);
    floor.material_mut().specular = 0.0;

    let mut wall = Box::new(Plane::new());
    *wall.transform_mut() = &(&Transform::translation(0.0, 0.0, 5.0)
        * &Transform::rotation_y(-std::f32::consts::FRAC_PI_4 as FLOAT))
        * &Transform::rotation_x(std::f32::consts::FRAC_PI_2 as FLOAT);
    *wall.material_mut() = floor.material().clone();

    let mut right_wall = Box::new(Plane::new());
    *right_wall.transform_mut() = &(&Transform::translation(0.0, 0.0, 5.0)
        * &Transform::rotation_y(std::f32::consts::FRAC_PI_4 as FLOAT))
        * &Transform::rotation_x(std::f32::consts::FRAC_PI_2 as FLOAT);
    *right_wall.material_mut() = floor.material().clone();

    let mut middle = Box::new(Sphere::new());
    *middle.transform_mut() = Transform::translation(-0.5, 1.0, 0.5);
    middle.material_mut().color = Color::new(0.1, 1.0, 0.5);
    middle.material_mut().diffuse = 0.7;

    let mut right = Box::new(Sphere::new());
    *right.transform_mut() = &Transform::translation(1.5, 0.5, -0.5)
        * &Transform::scaling(0.5, 0.5, 0.5);
    right.material_mut().color = Color::new(0.5, 1.0, 0.1);
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;

    let mut left = Box::new(Sphere::new());
    *left.transform_mut() = &Transform::translation(-1.5, 0.33, -0.75)
        * &Transform::scaling(0.33, 0.33, 0.33);
    left.material_mut().color = Color::new(1.0, 0.8, 0.1);
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;

    world.add_shape(floor);
    world.add_shape(wall);
    world.add_shape(right_wall);
    world.add_shape(middle);
    world.add_shape(right);
    world.add_shape(left);
    world.add_light(Light::new(
        Point3D::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera =
        Camera::new(300, 150, std::f32::consts::FRAC_PI_3 as FLOAT);
    *camera.transform_mut() = Transform::view_transform(
        &Point3D::new(0.0, 1.5, -5.0),
        &Point3D::new(0.0, 1.0, 0.0),
        &Vector3D::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    canvas.to_ppm(&mut writer).expect("write failed");
}
