extern crate hsl;
extern crate image;
extern crate num_complex;
extern crate rayon;
extern crate simple_easing;

use hsl::HSL;
use num_complex::Complex;
use rayon::prelude::*;
use simple_easing::{cubic_in_out, sine_out};

const X_MIN: f64 = -2.;
const X_MAX: f64 = 1.;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;

fn mandel(x: f64, y: f64, iter: usize) -> usize {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f64, 0f64);
    let mut i = 0usize;
    for t in 0..iter {
        if z.norm() > 2. {
            break;
        }
        z = z * z + c;
        i = t;
    }
    return i;
}

fn process_chunk((y, row): (usize, &mut [u8]), params: &Params) {
    let Params {
        img_size,
        max_iter,
        scalex,
        scaley,
        base,
        ..
    } = *params;
    for x in 0..img_size {
        // Get iteration count
        let cx = X_MIN + x as f64 * scalex;
        let cy = Y_MIN + y as f64 * scaley;
        let i = mandel(cx, cy, max_iter);
        let mut col = image::Rgba([0u8, 0u8, 0u8, 255u8]);
        // Convert iteration count to pixel color
        if i < max_iter - 1 {
            let c = (i as f64).log10() / base;
            // TODO: Use easing library that supports f64
            (col[0], col[1], col[2]) = HSL {
                h: 360. * cubic_in_out(c as f32) as f64,
                s: 0.8_f64,
                l: 1_f64 * sine_out(c as f32) as f64,
            }
            .to_rgb();
        }
        row[(x * 4) as usize] = col[0];
        row[(x * 4 + 1) as usize] = col[1];
        row[(x * 4 + 2) as usize] = col[2];
        row[(x * 4 + 3) as usize] = col[3];
    }
}

struct Params {
    img_size: usize,
    max_iter: usize,
    scalex: f64,
    scaley: f64,
    base: f64,
}

fn build_params(img_size: usize, max_iter: usize) -> Params {
    Params {
        img_size,
        max_iter,
        scalex: (X_MAX - X_MIN) / img_size as f64,
        scaley: (Y_MAX - Y_MIN) / img_size as f64,
        base: ((max_iter - 1) as f64).log10(),
    }
}

fn main() {
    // Params
    let params = build_params(1024 * 4, 1024 * 4);
    // Create image buffer
    let size = params.img_size as u32;
    let imgbuf = image::RgbaImage::new(size, size);
    let mut buffer = imgbuf.into_raw();
    println!("Start");
    // Calculate for each pixel
    buffer
        .par_chunks_mut(params.img_size * 4)
        .enumerate()
        .for_each(|t| process_chunk(t, &params));
    // Save image
    println!("Done, saving png...");
    let img = image::RgbaImage::from_raw(size, size, buffer).unwrap();
    img.save("fractal.png").unwrap();
}
