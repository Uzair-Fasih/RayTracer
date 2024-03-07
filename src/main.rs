mod vec3;
mod color;
mod ray;

use vec3::{Point3, Vec3};
use std::cmp;
use log::{info};
use ray::Ray;

use color::Color;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc: Vec3 = r.orig - center;
    let a = r.dirc.dot(r.dirc);
    let b = 2.0 * oc.dot(r.dirc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {

    if hit_sphere(Point3::new(0.0, 0.0, -1.0 as f64), 0.5, r) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = r.dirc.clone().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

}

fn main() {
    env_logger::init();

    // Aspect Ratio
    let aspect_ratio = 16.0 / 9.0;

    // Image
    let image_width = 400;
    let image_height = cmp::max(1, (image_width as f64 / aspect_ratio) as u32);
    
    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculating the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculating the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + pixel_delta_u + pixel_delta_v * 0.5;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);
            pixel_color.write_color();
        }
    }
    info!("Done.");
}