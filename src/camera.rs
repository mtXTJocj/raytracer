use super::transform::Transform;

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
}
