use super::transform::Transform;

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Transform,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Transform::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
