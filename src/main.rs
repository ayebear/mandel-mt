use clap::Parser;
use hsl::HSL;
use num_complex::Complex;
use rayon::prelude::*;
use simple_easing::{cubic_in_out, sine_out};

const DEF_LEFT: f64 = -2.9;
const DEF_RIGHT: f64 = 1.9;
const DEF_TOP: f64 = -1.35;
const DEF_BOTTOM: f64 = 1.35;
const DEF_SATURATION: f64 = 0.8;
const DEF_WIDTH: usize = 1920;
const DEF_HEIGHT: usize = 1080;
const DEF_MAX_ITERS: usize = 1024 * 4;
const DEF_FILENAME: &str = "fractal.png";
const DEF_HUE_SHIFT: f64 = 0.5;
const DEF_POWER: f64 = 2.;

/// Multi-threaded mandelbrot fractal generator in rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the image in pixels
    #[clap(short, long, value_parser, default_value_t = DEF_WIDTH)]
    width: usize,

    /// Height of the image in pixels
    #[clap(short, long, value_parser, default_value_t = DEF_HEIGHT)]
    height: usize,

    /// Maximum number of iterations
    #[clap(short, long, value_parser, default_value_t = DEF_MAX_ITERS)]
    iter: usize,

    /// Output filename for image
    #[clap(short, long, value_parser, default_value_t = DEF_FILENAME.to_string())]
    out: String,

    /// Amount to shift hue (between 0-1)
    #[clap(long, value_parser, default_value_t = DEF_HUE_SHIFT)]
    hue: f64,

    /// Color saturation amount (between 0-1)
    #[clap(short, long, value_parser, default_value_t = DEF_SATURATION)]
    saturation: f64,

    /// Power to use in fractal generation formula
    #[clap(short, long, value_parser, default_value_t = DEF_POWER)]
    power: f64,

    /// Multi-threading is enabled by default. Can disable by passing false. Useful to disable to avoid oversubscribing threads with a parallel runner.
    #[clap(long, value_parser, default_value_t = true)]
    mt: bool,

    /// Left coordinate of fractal space (min x)
    #[clap(short, long, value_parser, default_value_t = DEF_LEFT)]
    left: f64,

    /// Right coordinate of fractal space (max x)
    #[clap(short, long, value_parser, default_value_t = DEF_RIGHT)]
    right: f64,

    /// Top coordinate of fractal space (min y)
    #[clap(short, long, value_parser, default_value_t = DEF_TOP)]
    top: f64,

    /// Bottom coordinate of fractal space (max y)
    #[clap(short, long, value_parser, default_value_t = DEF_BOTTOM)]
    bottom: f64,
}

fn main() {
    // Parse params
    let args = Args::parse();
    let params = build_params(&args);
    // Create image buffer
    let w = args.width as u32;
    let h = args.height as u32;
    let imgbuf = image::RgbaImage::new(w, h);
    let mut buffer = imgbuf.into_raw();
    // Calculate for each row, multi-threaded or single-threaded based on args
    let chunk_size = args.width * 4;
    let processor = |t| process_chunk(t, &args, &params);
    if args.mt {
        buffer
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(processor);
    } else {
        buffer
            .chunks_mut(chunk_size)
            .enumerate()
            .for_each(processor);
    }
    // Save image
    println!("Writing {}", args.out);
    let img = image::RgbaImage::from_raw(w, h, buffer).unwrap();
    img.save(args.out).unwrap();
}

struct Params {
    scalex: f64,
    scaley: f64,
    base: f64,
}

fn build_params(args: &Args) -> Params {
    Params {
        scalex: (args.right - args.left) / args.width as f64,
        scaley: (args.bottom - args.top) / args.height as f64,
        base: ((args.iter - 1) as f64).log10(),
    }
}

fn mandel(x: f64, y: f64, iter: usize, power: f64) -> Option<usize> {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f64, 0f64);
    for i in 0..iter {
        if z.norm() > 2. {
            return Some(i);
        }
        z = z.powf(power) + c;
    }
    None
}

fn process_chunk((y, row): (usize, &mut [u8]), args: &Args, params: &Params) {
    let Params {
        scalex,
        scaley,
        base,
    } = *params;
    let Args {
        width,
        iter,
        left,
        top,
        hue,
        saturation,
        power,
        ..
    } = *args;
    for x in 0..width {
        // Get iteration count
        let cx = left + x as f64 * scalex;
        let cy = top + y as f64 * scaley;
        let result = mandel(cx, cy, iter, power);
        // Convert iteration count to pixel color
        let col = match result {
            Some(i) => {
                let c = ((i + 1) as f64).log10() / base;
                // TODO: Use easing library that supports f64
                // TODO: Use logarithmic curve instead of cubic
                // TODO: Use smoothstep algo to get smoother colors
                let e = cubic_in_out(c as f32) as f64;
                HSL {
                    h: (360. * (e + hue)) % 360.,
                    s: saturation,
                    l: sine_out(c as f32) as f64,
                }
                .to_rgb()
            }
            // TODO: Fill inner colors with something other than black
            None => (0, 0, 0),
        };
        // Store color in image buffer
        row[(x * 4) as usize] = col.0;
        row[(x * 4 + 1) as usize] = col.1;
        row[(x * 4 + 2) as usize] = col.2;
        row[(x * 4 + 3) as usize] = 255u8;
    }
}
