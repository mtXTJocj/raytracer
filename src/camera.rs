use super::{
    canvas::Canvas, point3d::Point3D, ray::Ray, transform::Transform,
};

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Transform,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / (vsize as f32);
        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / (hsize as f32);

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Transform::identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f32 + 0.5) * self.pixel_size;
        let yoffset = (py as f32 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let world_view = self.transform.inv();
        let pixel = world_view * &Point3D::new(world_x, world_y, -1.0);
        let origin = world_view * &Point3D::new(0.0, 0.0, 0.0);
        let mut direction = &pixel - &origin;
        direction.normalize();

        return Ray::new(origin, direction);
    }
}

#[cfg(test)]
mod tests {
    use super::{super::vector3d::Vector3D, *};

    #[test]
    fn constructing_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f32::consts::FRAC_PI_2;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(160, c.hsize);
        assert_eq!(120, c.vsize);
        assert_eq!(std::f32::consts::FRAC_PI_2, c.field_of_view);
        assert_eq!(Transform::identity(), c.transform);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f32::consts::FRAC_PI_2);
        assert_eq!(0.01, c.pixel_size);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, std::f32::consts::FRAC_PI_2);
        assert_eq!(0.01, c.pixel_size);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(Point3D::new(0.0, 0.0, 0.0), *r.origin());
        assert_eq!(Vector3D::new(0.0, 0.0, -1.0), *r.direction());
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, std::f32::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(Point3D::new(0.0, 0.0, 0.0), *r.origin());
        assert_eq!(Vector3D::new(0.66519, 0.33259, -0.66851), *r.direction());
    }
}
