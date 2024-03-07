mod vec3;
use vec3::Vec3;
use log::{info};

fn main() {
    env_logger::init();
    let v1 = Vec3::new(1 as f64, 2 as f64, 3 as f64);
    let v2 = Vec3::new(3 as f64, 2 as f64, 1 as f64);
    // We can only access value directly because it is public
    info!("{:?}", ((v1 + v2) * (4 as f64)).length());

    // Image
    let image_width: u16 = 256;
    let image_height: u16 = 256;
    
    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    
    for j in 0..image_height {
        
        // info!("Scanlines remaining: {}", (image_height - j));

        for i in 0..image_width {
            let r: f32 = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0 as f32;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib)
        }
    }
    
    info!("Done.");
}
