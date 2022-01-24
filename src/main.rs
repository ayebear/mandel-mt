extern crate image;
extern crate num_complex;

use num_complex::Complex;

fn mandel(x: f32, y: f32, iter: u32) -> u32 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f32, 0f32);
    let mut i = 0u32;
    for t in 0..iter {
        if z.norm() > 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    return i;
}

fn main() {
    let max_iterations = 1024u32;
    let img_size = 512u32;
    let cxmin = -2f32;
    let cxmax = 1f32;
    let cymin = -1.5f32;
    let cymax = 1.5f32;
    let scalex = (cxmax - cxmin) / img_size as f32;
    let scaley = (cymax - cymin) / img_size as f32;
    let base = ((max_iterations - 1) as f32).log10();
    // Create a new ImgBuf
    let mut imgbuf = image::ImageBuffer::new(img_size, img_size);
    // Calculate for each pixel
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Get iteration count
        let cx = cxmin + x as f32 * scalex;
        let cy = cymin + y as f32 * scaley;
        let i = mandel(cx, cy, max_iterations);
        let mut col = image::Rgb([0u8, 0u8, 0u8]);
        // Convert iteration count to pixel color
        if i < max_iterations - 1 {
            let c = (3.0 * (i as f32).log10()) / base;
            if c < 1.0 {
                col[2] = (255.0 * c) as u8;
            } else if c < 2.0 {
                col[1] = (255.0 * (c - 1.0)) as u8;
                col[2] = 255u8;
            } else {
                col[0] = 255u8;
                col[1] = (255.0 * (c - 2.0)) as u8;
                col[2] = 255u8;
            }
        }
        *pixel = col;
    }
    // Save image
    imgbuf.save("fractal.png").unwrap();
}
