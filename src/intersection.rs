use super::sphere::Sphere;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection { t: 3.5, object: &s };

        assert_eq!(3.5, i.t);
        assert!(std::ptr::eq(&s, i.object));
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let xs = vec![i1, i2];

        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }
}
