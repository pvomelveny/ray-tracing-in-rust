pub type Color = crate::vec3::Vec3;

impl Color {
    pub fn r(self) -> f64 {
        self.x()
    }

    pub fn g(self) -> f64 {
        self.y()
    }

    pub fn b(self) -> f64 {
        self.z()
    }

    pub fn to_ppm_pix(self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self.r()) as u64,
            (255.999 * self.g()) as u64,
            (255.999 * self.b()) as u64,
        )
    }
}
