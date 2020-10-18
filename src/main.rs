mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

fn main() {
    render_ppm();
}

fn render_ppm() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio).floor() as i64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0, 0, 0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2) - (vertical / 2) - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + (u * horizontal) + (v * vertical) - origin,
            );
            let pixel_color = ray_color(r);
            println!("{}", pixel_color.to_ppm_pix());
        }
    }
    eprintln! {"Done"};
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point3::new(0, 0, -1), 0.5, ray) {
        return Color::new(1, 0, 0);
    }

    let unit_direction = ray.dir().unit();

    let t = 0.5 * (unit_direction.y() + 1.0);

    ((1. - t) * Color::new(1, 1, 1)) + (t * Color::new(0.5, 0.7, 1.0))
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.clone().origin() - center;
    let a = Vec3::dot(r.dir(), r.dir());
    let b = 2.0 * Vec3::dot(oc, r.dir());
    let c = Vec3::dot(oc, oc) - (radius * radius);

    let discriminant = (b * b) - (4.0 * a * c);

    discriminant > 0.0
}
