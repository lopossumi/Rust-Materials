use crate::vector::*;
use crate::hittable::*;
use crate::hittable_list::*;
use rand::prelude::*;

const T_MIN: f32 = 0.0001;
const T_MAX: f32 = f32::MAX;

const COLOR_WHITE: Color = Color { x: 1.0, y: 1.0, z: 1.0};
const COLOR_SKYBLUE: Color = Color { x: 0.5, y: 0.7, z: 1.0 };

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, distance: f32) -> Point3 {
        self.origin + distance * self.direction
    }

    pub fn color(&self, world: &HittableList, rng: &mut ThreadRng, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let t = world.hit(&self, T_MIN, T_MAX);
        match t {
            Some(record) => {
                let target = record.p + record.normal + Vec3::random(rng);
                0.5 * Ray::new(record.p, target-record.p).color(world, rng, depth - 1)
                //(COLOR_WHITE + record.normal.unit_vector())
            }

            None => {
                let unit_direction = self.direction.unit_vector();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * COLOR_WHITE + t * COLOR_SKYBLUE
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected.x, $actual.x, tolerance);
            assert_approx_eq!($expected.y, $actual.y, tolerance);
            assert_approx_eq!($expected.z, $actual.z, tolerance);
        };
    }

    #[test]
    fn at_distance() {
        let ray = Ray::new(Point3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 4.0, 0.0));
        let position = ray.at(5.0);
        let expected = Vec3::new(16.0, 21.0, 1.0);
        assert_vec3_equal!(expected, position);
    }
}
