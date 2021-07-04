use raytracer::{
    camera::Camera, color::Color, light::Light, node::Node,
    obj_file::parse_obj_file, point3d::Point3D, transform::Transform,
    vector3d::Vector3D, world::World, FLOAT,
};

use std::{
    boxed::Box,
    env,
    fs::File,
    io::{stdout, BufReader, BufWriter, Write},
};

fn main() {
    let mut writer: Box<dyn Write> = match env::args().nth(1) {
        None => Box::new(BufWriter::new(stdout())),
        Some(filename) => Box::new(BufWriter::new(
            File::create(filename).expect("cannot create file"),
        )),
    };

    let mut world = World::new();

    let mut reader = BufReader::new(
        File::open("./teapot-low.obj").expect("cannot open file"),
    );
    let parser = parse_obj_file(&mut reader);
    let mut group: Box<Node> = parser.into();
    group.set_transform(
        &Transform::translation(0.0, -10.0, 0.0)
            * &Transform::rotation_x(-std::f64::consts::FRAC_PI_2 as FLOAT),
    );

    world.add_node(group);

    world.add_light(Light::new(
        Point3D::new(-10.0, 20.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera =
        Camera::new(600, 300, std::f32::consts::FRAC_PI_3 as FLOAT);
    *camera.transform_mut() = Transform::view_transform(
        &Point3D::new(0.0, 3.5, -50.0),
        &Point3D::new(0.0, 0.0, 0.0),
        &Vector3D::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    canvas.to_ppm(&mut writer).expect("write failed");
}
