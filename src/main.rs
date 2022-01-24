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
    let img_side = 128u32;
    let cxmin = -2f32;
    let cxmax = 1f32;
    let cymin = -1.5f32;
    let cymax = 1.5f32;
    let scalex = (cxmax - cxmin) / img_side as f32;
    let scaley = (cymax - cymin) / img_side as f32;
    // Create a new ImgBuf
    let mut imgbuf = image::ImageBuffer::new(img_side, img_side);
    // Calculate for each pixel
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = cxmin + x as f32 * scalex;
        let cy = cymin + y as f32 * scaley;
        let i = mandel(cx, cy, max_iterations);
        let col = ((i as f32 / max_iterations as f32) * 256f32).floor() as u8;
        // *pixel = image::Luma([col]);
        *pixel = image::Rgb([col, col, col]);
    }
    // Save image
    imgbuf.save("fractal.png").unwrap();
}
