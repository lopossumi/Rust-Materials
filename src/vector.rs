use core::fmt;
use std::ops::{Add, Sub, Neg, Mul, Div};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x 
        + self.y * other.y 
        + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x 
        + self.y * self.y 
        + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        fn f(num: f64) -> u8 {
            if num < 0.0 { 
                0
            }
            else if num >= 1.0 {
                255
            }
            else {
                (num * 255.99) as u8
            }
        }
        [f(self.x), f(self.y), f(self.z)]
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{
            x: -self.x,
            y: -self.y, 
            z: -self.z
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3{
            x: self * vector.x,
            y: self * vector.y, 
            z: self * vector.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3{
            x: self.x * scalar,
            y: self.y * scalar, 
            z: self.z * scalar
        }
    }
}

// Hadamard product
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x * other.x,
            y: self.y * other.y, 
            z: self.z * other.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3 {
        Vec3{
            x: self.x / scalar,
            y: self.y / scalar, 
            z: self.z / scalar
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::*;
    use super::*;

    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected.x, $actual.x, tolerance);
            assert_approx_eq!($expected.y, $actual.y, tolerance);
            assert_approx_eq!($expected.z, $actual.z, tolerance);
        };
    }
    
    #[test]
    fn addition() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vector1 + vector2;
        let expected = Vec3::new(5.0, 7.0, 9.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn substraction() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vector1 - vector2;
        let expected = Vec3::new(-3.0, -3.0, -3.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn negation() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let result = -vector1; 
        let expected = Vec3::new(-1.0, -2.0, -3.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn multiplication() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let result1 = 5.0 * vector1;
        let expected = Vec3::new(5.0, 10.0, 15.0);
        assert_vec3_equal!(expected, result1);

        let result2 = vector1 * 5.0;
        assert_vec3_equal!(expected, result2);
    }

    #[test]
    fn hadamard_product() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);

        let result = vector1 * vector2;
        let expected = Vec3::new(4.0, 10.0, 18.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn division() {
        let vector = Vec3::new(12.0, 13.5, 15.0);
        let divisor = 3.0;
        let result = vector / divisor;
        let expected = Vec3::new(4.0, 4.5, 5.0);
        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn length() {
        let vector = Vec3::new(3.0, 4.0, 5.0);
        let result = vector.length();
        assert_approx_eq!(7.07168, result, 0.001);
    }

    #[test]
    fn length_squared() {
        let vector = Vec3::new(3.0, 4.0, 5.0);
        let result = vector.length_squared();
        assert_approx_eq!(50.0, result, 0.001);
    }

    #[test]
    fn dot(){
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(1.0, 5.0, 7.0);
        let result = vector1.dot(vector2);
        assert_approx_eq!(32.0, result, 0.001);
    }

    #[test]
    fn cross(){
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(1.0, 5.0, 7.0);
        let result = vector1.cross(vector2);
        let expected = Vec3::new(-1.0, -4.0, 3.0);
        assert_vec3_equal!(expected, result);

    }

    #[test]
    fn to_rgb(){
        let vector = Vec3::new(-1.0, 0.5, 1.4);
        let result = vector.to_rgb();
        let expected = [0u8, 127u8, 255u8];
        assert_eq!(expected, result);
    }

    #[test]
    fn unit_vector(){
        let vector = Vec3::new(-1.0, 0.5, 1.4);
        let result = vector.unit_vector();
        let expected = Vec3::new(-0.55815, 0.27907, 0.78140);
        assert_vec3_equal!(expected, result);
    }
}
