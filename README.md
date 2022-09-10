# mandel-mt

Multi-threaded mandelbrot fractal generator in rust

## Usage

To view arguments, run:

```
cargo run -r -- --help
```

Which prints something like:

```
mandel-mt 0.1.0
Multi-threaded mandelbrot fractal generator in rust

USAGE:
    mandel-mt [OPTIONS]

OPTIONS:
    -h, --hue <HUE>      Amount to shift hue (between 0-1) [default: 0.5]
        --help           Print help information
    -i, --iter <ITER>    Maximum number of iterations [default: 4096]
    -o, --out <OUT>      Output filename for image [default: fractal.png]
    -s, --size <SIZE>    Size of the image in pixels, used for both width and height [default: 2048]
    -V, --version        Print version information
```

## Output

When run with default settings:

![fractal image](https://github.com/ayebear/mandel-mt/blob/master/example.png?raw=true)
