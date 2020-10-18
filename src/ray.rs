use crate::vec3::{Point3, Vec3};

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            origin: orig,
            direction: dir,
        }
    }

    pub fn origin(self) -> Point3 {
        self.origin
    }

    pub fn dir(self) -> Vec3 {
        self.direction
    }

    pub fn at(self, t: f64) -> Point3 {
        return self.origin + (t * self.direction);
    }
}
