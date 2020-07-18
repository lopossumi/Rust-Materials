use crate::vector::*;
use crate::ray::*;
use crate::hittable::{ Record, Hittable };

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { 
            center: center, 
            radius : radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        
        if discriminant > 0.0 {

            let root = discriminant.sqrt();
            let mut temp = (-half_b - root)/a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                return Some(Record::new(p, outward_normal, t, front_face));
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                return Some(Record::new(p, outward_normal, t, front_face));
            }
        }

        None  
    }
}