mod vec3;
mod color;

use vec3::Vec3;
use log::{info};

use color::Color;
use crate::color::Utility;

fn main() {
    env_logger::init();

    // Image
    let image_width: u16 = 256;
    let image_height: u16 = 256;
    
    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    
    for j in 0..image_height {
        
        info!("Scanlines remaining: {}", (image_height - j));

        for i in 0..image_width {
            let r: f64 = (i as f64) / ((image_width - 1) as f64);
            let g: f64 = (j as f64) / ((image_height - 1) as f64);
            let b: f64 = 0 as f64;

            Color::new(r, g, b).write_color()
        }
    }
    
    info!("Done.");
}
