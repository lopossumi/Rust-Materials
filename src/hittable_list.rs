use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;
use crate::vector::*;

pub struct HittableList {
    pub objects: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        let mut spheres = Vec::new();
        spheres.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
        spheres.push(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0));
        HittableList { objects: spheres }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Record> {
        let mut temp_rec = Record::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            t_max,
            true,
        );

        for object in &self.objects {
            match object.hit(ray, t_min, t_max) {
                Some(record) => {
                    if record.t < temp_rec.t {
                        temp_rec = record;
                    }
                }
                None => {}
            }
        }

        if temp_rec.t == t_max {
            None
        } else {
            Some(temp_rec)
        }
    }
}
