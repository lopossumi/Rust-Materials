use crate::ray::Ray;
use crate::vector::*;

pub struct Record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl Record {
    pub fn new(p: Point3, normal: Vec3, t: f32, front_face: bool) -> Record {
        Record {
            p: p,
            normal: if front_face { normal } else { -normal },
            t: t,
            front_face: front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Record>;
}