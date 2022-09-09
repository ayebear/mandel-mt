extern crate hsl;
extern crate image;
extern crate num_complex;
extern crate rayon;

use hsl::HSL;
use num_complex::Complex;
use rayon::prelude::*;

fn mandel(x: f64, y: f64, iter: u64) -> u64 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f64, 0f64);
    let mut i = 0u64;
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
    // Params
    let max_iterations = 1024u64 * 4;
    let img_size = 1024u32 * 4;
    let cxmin = -2f64;
    let cxmax = 1f64;
    let cymin = -1.5f64;
    let cymax = 1.5f64;
    let scalex = (cxmax - cxmin) / img_size as f64;
    let scaley = (cymax - cymin) / img_size as f64;
    let base = ((max_iterations - 1) as f64).log10();
    // Create image buffer
    let imgbuf = image::RgbaImage::new(img_size, img_size);
    let mut buffer = imgbuf.into_raw();
    println!("Start");
    // Calculate for each pixel
    buffer
        .par_chunks_mut((img_size * 4) as usize)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..img_size {
                // Get iteration count
                let cx = cxmin + x as f64 * scalex;
                let cy = cymin + y as f64 * scaley;
                let i = mandel(cx, cy, max_iterations);
                let mut col = image::Rgba([0u8, 0u8, 0u8, 255u8]);
                // Convert iteration count to pixel color
                if i < max_iterations - 1 {
                    let c = (360.0 * (i as f64).log10()) / base;
                    let (r, g, b) = HSL {
                        h: c,
                        s: 1_f64,
                        l: 0.5_f64,
                    }
                    .to_rgb();
                    col[0] = r;
                    col[1] = g;
                    col[2] = b;
                }
                // imgbuf.put_pixel(x, y, col);
                row[(x * 4) as usize] = col[0];
                row[(x * 4 + 1) as usize] = col[1];
                row[(x * 4 + 2) as usize] = col[2];
                row[(x * 4 + 3) as usize] = col[3];
            }
        });
    // Save image
    println!("Done, saving png...");
    let img = image::RgbaImage::from_raw(img_size, img_size, buffer).unwrap();
    img.save("fractal.png").unwrap();
}
