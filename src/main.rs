fn main() {
    // Image
    let image_width: u16 = 256;
    let image_height: u16 = 256;
    
    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    
    for j in 0..image_width {
        for i in 0..image_height {
            let r: f32 = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0 as f32;

            let ir: u8 = (255.999 * r) as u8;
            let ig: u8 = (255.999 * g) as u8;
            let ib: u8 = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib)
        }
    }
}
