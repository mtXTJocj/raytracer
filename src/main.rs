use raytracer::{
    canvas::Canvas, color::Color, intersection::hit, point3d::Point3D,
    ray::Ray, sphere::Sphere,
};

use std::{
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

    let ray_origin = Point3D::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels: usize = 100;
    let size_per_pixel = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::RED;
    let shape = Sphere::new();

    for y in 0..canvas.height() {
        let world_y = half - (size_per_pixel * y as f32);

        for x in 0..canvas.width() {
            let world_x = -half + (size_per_pixel * x as f32);
            let position = Point3D::new(world_x, world_y, wall_z);
            let mut direction = &position - &ray_origin;
            direction.normalize();

            let r = Ray::new(ray_origin.clone(), direction);
            let xs = shape.intersect(&r);

            if let Some(_) = hit(&xs) {
                *canvas.color_at_mut(x, y) = color;
            }
        }
    }
    canvas.to_ppm(&mut writer).expect("write failed");
}
