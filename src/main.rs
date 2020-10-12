mod vec3;

fn main() {
    write_ppm();
    let x = vec3::Vec3::new(4, 5, 6);

    let y = vec3::Vec3::new(1, 2, 3);
    let z = x + y;
    eprintln!("{:?}", z);
}

fn write_ppm() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines Remaining: {}", j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln! {"Done"};
}
