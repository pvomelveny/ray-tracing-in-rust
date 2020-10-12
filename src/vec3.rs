use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Vec3 {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}
// Addition
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
// Subtraction
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Vec3 {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
// Multiplication (Scalar)
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}
impl Mul<i64> for Vec3 {
    type Output = Self;

    fn mul(self, s: i64) -> Self {
        Self {
            x: self.x * (s as f64),
            y: self.y * (s as f64),
            z: self.z * (s as f64),
        }
    }
}
impl Mul<Vec3> for i64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}
