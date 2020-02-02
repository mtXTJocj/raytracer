use super::sphere::Sphere;

/// Ray とオブジェクトとの交点
#[derive(Debug)]
pub struct Intersection<'a> {
    /// 交差する Ray の始点からの距離
    pub t: f32,
    /// Ray と交差したオブジェクト
    pub object: &'a Sphere,
}

/// 複数の交点のうち、Ray の始点よりも先で最も手前にあるものを返す。
/// 存在しない場合は None を返す。
///
/// # Argumets
/// * `xs` - 候補となる Intersection の Vec
pub fn hit<'a, 'b>(
    xs: &'a Vec<Intersection<'b>>,
) -> Option<&'a Intersection<'b>> {
    let mut min_t = std::f32::MAX;
    let mut result = None;

    for x in xs {
        if 0.0 <= x.t && x.t < min_t {
            min_t = x.t;
            result = Some(x)
        }
    }
    result
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

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };
        let xs = vec![i2, i1];

        if let Some(i) = hit(&xs) {
            assert!(std::ptr::eq(i, &xs[1]));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection {
            t: -1.0,
            object: &s,
        };
        let i2 = Intersection { t: 1.0, object: &s };
        let xs = vec![i2, i1];

        if let Some(i) = hit(&xs) {
            assert!(std::ptr::eq(i, &xs[0]));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection {
            t: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            t: -1.0,
            object: &s,
        };
        let xs = vec![i2, i1];

        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };
        let xs = vec![i1, i2, i3, i4];

        if let Some(i) = hit(&xs) {
            assert!(std::ptr::eq(i, &xs[3]));
        } else {
            assert!(false);
        }
    }
}
