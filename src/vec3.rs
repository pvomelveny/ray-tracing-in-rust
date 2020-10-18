use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Vec3 {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(v: Vec3, w: Vec3) -> f64 {
        (v.x * w.x) + (v.y * w.y) + (v.z * w.z)
    }

    pub fn hammard(v: Vec3, w: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * w.x,
            y: v.y * w.y,
            z: v.z * w.z,
        }
    }

    pub fn cross(v: Vec3, w: Vec3) -> Vec3 {
        Vec3 {
            x: (v.y * w.z) - (w.y * v.z),
            y: (v.z * w.x) - (v.x * w.z),
            z: (v.x * w.y) - (v.y * w.x),
        }
    }
}

// Display
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
/*
Convenience Operators
*/
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
// Division
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, s: f64) -> Self {
        (1. / s) * self
    }
}
impl Div<i64> for Vec3 {
    type Output = Self;

    fn div(self, s: i64) -> Self {
        (1 / s) * self
    }
}
// Negative
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
