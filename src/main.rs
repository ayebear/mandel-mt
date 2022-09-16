use clap::Parser;
use hsl::HSL;
use num_complex::Complex;
use rayon::prelude::*;
use simple_easing::{cubic_in_out, sine_out};

const X_MIN: f64 = -2.;
const X_MAX: f64 = 1.;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;
const SATURATION: f64 = 0.8;
const DEF_IMG_SIZE: usize = 1024 * 2;
const DEF_MAX_ITERS: usize = 1024 * 4;
const DEF_FILENAME: &str = "fractal.png";
const DEF_HUE_SHIFT: f64 = 0.5;
const DEF_POWER: f64 = 2.;

/// Multi-threaded mandelbrot fractal generator in rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Size of the image in pixels, used for both width and height
    #[clap(short, long, value_parser, default_value_t = DEF_IMG_SIZE)]
    size: usize,

    /// Maximum number of iterations
    #[clap(short, long, value_parser, default_value_t = DEF_MAX_ITERS)]
    iter: usize,

    /// Output filename for image
    #[clap(short, long, value_parser, default_value_t = DEF_FILENAME.to_string())]
    out: String,

    /// Amount to shift hue (between 0-1)
    #[clap(short, long, value_parser, default_value_t = DEF_HUE_SHIFT)]
    hue: f64,

    /// Power to use in fractal generation formula
    #[clap(short, long, value_parser, default_value_t = DEF_POWER)]
    power: f64,
}

fn main() {
    // Parse params
    let args = Args::parse();
    let params = build_params(args.size, args.iter, args.out, args.hue, args.power);
    // Create image buffer
    println!("Start");
    let size = params.img_size as u32;
    let imgbuf = image::RgbaImage::new(size, size);
    let mut buffer = imgbuf.into_raw();
    // Calculate for each pixel
    buffer
        .par_chunks_mut(params.img_size * 4)
        .enumerate()
        .for_each(|t| process_chunk(t, &params));
    // Save image
    println!("Writing {}", params.filename);
    let img = image::RgbaImage::from_raw(size, size, buffer).unwrap();
    img.save(params.filename).unwrap();
}

struct Params {
    img_size: usize,
    max_iter: usize,
    filename: String,
    scalex: f64,
    scaley: f64,
    base: f64,
    hue_shift: f64,
    power: f64,
}

fn build_params(
    img_size: usize,
    max_iter: usize,
    filename: String,
    hue_shift: f64,
    power: f64,
) -> Params {
    Params {
        img_size,
        max_iter,
        filename,
        scalex: (X_MAX - X_MIN) / img_size as f64,
        scaley: (Y_MAX - Y_MIN) / img_size as f64,
        base: ((max_iter - 1) as f64).log10(),
        hue_shift,
        power,
    }
}

fn mandel(x: f64, y: f64, iter: usize, power: f64) -> usize {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f64, 0f64);
    let mut i = 0usize;
    for t in 0..iter {
        if z.norm() > 2. {
            break;
        }
        z = z.powf(power) + c;
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
        hue_shift,
        power,
        ..
    } = *params;
    for x in 0..img_size {
        // Get iteration count
        let cx = X_MIN + x as f64 * scalex;
        let cy = Y_MIN + y as f64 * scaley;
        let i = mandel(cx, cy, max_iter, power);
        let mut col = image::Rgba([0u8, 0u8, 0u8, 255u8]);
        // Convert iteration count to pixel color
        if i < max_iter - 1 {
            let c = (i as f64).log10() / base;
            // TODO: Use easing library that supports f64
            let e = cubic_in_out(c as f32) as f64;
            (col[0], col[1], col[2]) = HSL {
                h: (360. * (e + hue_shift)) % 360.,
                s: SATURATION,
                l: sine_out(c as f32) as f64,
            }
            .to_rgb();
        }
        row[(x * 4) as usize] = col[0];
        row[(x * 4 + 1) as usize] = col[1];
        row[(x * 4 + 2) as usize] = col[2];
        row[(x * 4 + 3) as usize] = col[3];
    }
}
