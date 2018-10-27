extern crate cfg_if;
extern crate hsl;
extern crate wasm_bindgen;

mod utils;
use wasm_bindgen::prelude::*;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const MAX_ITER: u8 = 255;

static mut TIME: usize = 0;
static mut MANDELBROT: Option<Vec<u8>> = None;
static mut VIEWPORT: f64 = 0.5;
static mut OFFSET_X: f64 = -0.29;
static mut OFFSET_Y: f64 = -1.05;

fn generate_mandelbrot(viewport: f64, offset_x: f64, offset_y: f64) -> Vec<u8> {
    let mut mandelbrot = Vec::with_capacity(WIDTH * HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let x0 = (x as f64) / (WIDTH as f64) * viewport + offset_x;
            let y0 = (y as f64) / (HEIGHT as f64) * viewport + offset_y;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iter = 0;
            while iter < MAX_ITER {
                if x * x + y * y > 2.0 * 2.0 {
                    break;
                }
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iter += 1;
            }
            mandelbrot.push(iter);
        }
    }
    mandelbrot
}

#[wasm_bindgen]
pub fn frame(frame_buffer: &mut [u8], key_down: bool) {
    utils::set_panic_hook();

    let time = unsafe {
        let t = TIME;
        TIME += 1;
        t
    };

    let mut mandelbrot = unsafe { MANDELBROT.take() };

    if key_down {
        unsafe {
            VIEWPORT /= 2.0;
            OFFSET_X += VIEWPORT / 2.0;
            OFFSET_Y += VIEWPORT / 2.0;
        }
        mandelbrot = None;
    }

    let mandelbrot =
        mandelbrot.unwrap_or_else(|| unsafe { generate_mandelbrot(VIEWPORT, OFFSET_X, OFFSET_Y) });

    for (pixel, iter) in frame_buffer.chunks_mut(4).zip(mandelbrot.iter()) {
        let color = hsl::HSL {
            h: ((((*iter as usize * 20 - time) % 360) + 360) % 360) as f64,
            s: 0.7,
            l: 0.7,
        }.to_rgb();

        pixel[0] = color.0;
        pixel[1] = color.1;
        pixel[2] = color.2;
        pixel[3] = 255;
    }

    unsafe {
        MANDELBROT = Some(mandelbrot);
    }
}
