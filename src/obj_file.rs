use crate::{
    group::Group, node::Node, point3d::Point3D, triangle::Triangle, FLOAT,
};
use std::{collections::BTreeMap, convert::From, io::BufRead};

#[derive(Debug)]
pub struct ObjParser {
    vertices: Vec<Point3D>,
    default_group: Box<Node>,
    groups: BTreeMap<String, Box<Node>>,
}

fn fan_triangulation(
    vertices: &Vec<Point3D>,
    indices: &Vec<usize>,
) -> Vec<Triangle> {
    let mut triangles = vec![];

    for i in 1..indices.len() - 1 {
        triangles.push(Triangle::new(
            vertices[indices[0]].clone(),
            vertices[indices[i]].clone(),
            vertices[indices[i + 1]].clone(),
        ));
    }

    triangles
}

pub fn parse_obj_file(reader: &mut dyn BufRead) -> ObjParser {
    let mut default_group = Node::new(Box::new(Group::new()));
    let mut groups = BTreeMap::new();

    // 1-origin にする
    let mut vertices: Vec<Point3D> = vec![Point3D::new(0.0, 0.0, 0.0)];

    {
        let mut current_group = &mut default_group;

        for line in reader.lines() {
            let l = line.unwrap();
            let cs: Vec<&str> = l.trim().split_whitespace().collect();
            if cs.len() == 0 {
                continue;
            }

            match cs[0] {
                // vertex
                "v" => {
                    if cs.len() >= 4 {
                        vertices.push(Point3D::new(
                            cs[1].parse::<FLOAT>().unwrap(),
                            cs[2].parse::<FLOAT>().unwrap(),
                            cs[3].parse::<FLOAT>().unwrap(),
                        ));
                    }
                }
                // face
                "f" => {
                    if cs.len() >= 4 {
                        let indices = cs[1..]
                            .into_iter()
                            .map(|i| {
                                let face: Vec<&str> = i.split('/').collect();
                                face[0].parse::<usize>().unwrap()
                            })
                            .collect();
                        let triangles = fan_triangulation(&vertices, &indices);
                        for t in triangles {
                            current_group.add_child(Node::new(Box::new(t)));
                        }
                    }
                }
                // group
                "g" => {
                    assert!(cs.len() >= 2);
                    let name = cs[1].to_string();
                    let g = Node::new(Box::new(Group::new()));
                    groups.insert(name, g);
                    current_group = groups.get_mut(cs[1]).unwrap();
                }
                _ => {}
            }
        }
    }

    ObjParser {
        vertices,
        default_group,
        groups,
    }
}

impl From<ObjParser> for Box<Node> {
    fn from(mut parser: ObjParser) -> Self {
        for (_, v) in parser.groups {
            parser.default_group.add_child(v);
        }
        parser.default_group
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignoring_unrecognized_lines() {
        let mut gibberish: &[u8] = br"There was a youg lady named Bright
who traveled much faster than light
She set out one day
in a relative way,
and came back the previous night.";

        parse_obj_file(&mut gibberish);
    }

    #[test]
    fn vertex_records() {
        let mut file: &[u8] = b"v -1 1 0
v -1.0000 0.50000 0.0000
v 1 0 0
v 1 1 0";

        let parser = parse_obj_file(&mut file);

        assert_eq!(Point3D::new(-1.0, 1.0, 0.0), parser.vertices[1]);
        assert_eq!(Point3D::new(-1.0, 0.5, 0.0), parser.vertices[2]);
        assert_eq!(Point3D::new(1.0, 0.0, 0.0), parser.vertices[3]);
        assert_eq!(Point3D::new(1.0, 1.0, 0.0), parser.vertices[4]);
    }

    #[test]
    fn parsing_triangle_faces() {
        let mut file: &[u8] = b"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4";

        let parser = parse_obj_file(&mut file);
        let g = &parser.default_group;
        let t1 = g.child_at(0);
        let t1 = t1.shape();
        let t1 = &(**t1) as *const _ as *const Triangle;
        let t2 = g.child_at(1);
        let t2 = t2.shape();
        let t2 = &(**t2) as *const _ as *const Triangle;

        assert_eq!(unsafe { (*t1).p1() }, &parser.vertices[1]);
        assert_eq!(unsafe { (*t1).p2() }, &parser.vertices[2]);
        assert_eq!(unsafe { (*t1).p3() }, &parser.vertices[3]);
        assert_eq!(unsafe { (*t2).p1() }, &parser.vertices[1]);
        assert_eq!(unsafe { (*t2).p2() }, &parser.vertices[3]);
        assert_eq!(unsafe { (*t2).p3() }, &parser.vertices[4]);
    }

    #[test]
    fn triangulating_polygons() {
        let mut file: &[u8] = b"v -1 1 0
    v -1 0 0
    v 1 0 0
    v 1 1 0
    v 0 2 0

    f 1 2 3 4 5";

        let parser = parse_obj_file(&mut file);
        let g = &parser.default_group;
        let t1 = g.child_at(0);
        let t1 = t1.shape();
        let t1 = &(**t1) as *const _ as *const Triangle;
        let t2 = g.child_at(1);
        let t2 = t2.shape();
        let t2 = &(**t2) as *const _ as *const Triangle;
        let t3 = g.child_at(2);
        let t3 = t3.shape();
        let t3 = &(**t3) as *const _ as *const Triangle;

        assert_eq!(unsafe { (*t1).p1() }, &parser.vertices[1]);
        assert_eq!(unsafe { (*t1).p2() }, &parser.vertices[2]);
        assert_eq!(unsafe { (*t1).p3() }, &parser.vertices[3]);
        assert_eq!(unsafe { (*t2).p1() }, &parser.vertices[1]);
        assert_eq!(unsafe { (*t2).p2() }, &parser.vertices[3]);
        assert_eq!(unsafe { (*t2).p3() }, &parser.vertices[4]);
        assert_eq!(unsafe { (*t3).p1() }, &parser.vertices[1]);
        assert_eq!(unsafe { (*t3).p2() }, &parser.vertices[4]);
        assert_eq!(unsafe { (*t3).p3() }, &parser.vertices[5]);
    }

    #[test]
    fn triangles_in_groups() {
        let mut file: &[u8] = b"v -1 1 0
    v -1 0 0
    v 1 0 0
    v 1 1 0

    g FirstGroup
    f 1 2 3
    g SecondGroup
    f 1 3 4";

        let parser = parse_obj_file(&mut file);
        let g1 = parser.groups.get("FirstGroup").unwrap();
        let g2 = parser.groups.get("SecondGroup").unwrap();
        let t1 = g1.child_at(0);
        let t1 = t1.shape();
        let t1 = &(**t1) as *const _ as *const Triangle;
        let t2 = g2.child_at(0);
        let t2 = t2.shape();
        let t2 = &(**t2) as *const _ as *const Triangle;

        assert_eq!(unsafe { (*t1).p1() }, &parser.vertices[1]);
        assert_eq!(unsafe { (*t1).p2() }, &parser.vertices[2]);
        assert_eq!(unsafe { (*t1).p3() }, &parser.vertices[3]);
        assert_eq!(unsafe { (*t2).p1() }, &parser.vertices[1]);
        assert_eq!(unsafe { (*t2).p2() }, &parser.vertices[3]);
        assert_eq!(unsafe { (*t2).p3() }, &parser.vertices[4]);
    }

    #[test]
    fn converting_an_obj_file_to_a_group() {
        let mut file: &[u8] = b"v -1 1 0
    v -1 0 0
    v 1 0 0
    v 1 1 0

    g FirstGroup
    f 1 2 3
    g SecondGroup
    f 1 3 4";

        let parser = parse_obj_file(&mut file);
        let v1 = parser.vertices[1].clone();
        let v2 = parser.vertices[2].clone();
        let v3 = parser.vertices[3].clone();
        let v4 = parser.vertices[4].clone();
        let group: Box<Node> = parser.into();
        let g1 = group.child_at(0);
        let g2 = group.child_at(1);
        let t1 = g1.child_at(0);
        let t1 = t1.shape();
        let t1 = &(**t1) as *const _ as *const Triangle;
        let t2 = g2.child_at(0);
        let t2 = t2.shape();
        let t2 = &(**t2) as *const _ as *const Triangle;

        assert_eq!(unsafe { (*t1).p1() }, &v1);
        assert_eq!(unsafe { (*t1).p2() }, &v2);
        assert_eq!(unsafe { (*t1).p3() }, &v3);
        assert_eq!(unsafe { (*t2).p1() }, &v1);
        assert_eq!(unsafe { (*t2).p2() }, &v3);
        assert_eq!(unsafe { (*t2).p3() }, &v4);
    }
}
