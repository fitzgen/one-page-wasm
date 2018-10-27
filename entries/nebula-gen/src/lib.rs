extern crate cfg_if;
extern crate js_sys;
extern crate lazy_static;
extern crate wasm_bindgen;

mod utils;

use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const NUM_CELLS: usize = 64;
const MAX_VELOCITY: f64 = 1.25;

// Distance between two points.
fn distance(a: [f64; 2], b: [f64; 2]) -> f64 {
    ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt()
}

// Make a vector field from cellular noise. Generate some random center points
// for the cells. Then for each pixel, find its closest cell center point. Set
// the corresponding entry in the vector field to the x and y distance between
// that cell center point and the pixel.
fn make_distance_vector_field() -> Vec<[f64; 2]> {
    let cell_points: Vec<_> = (0..NUM_CELLS).map(|_| random_pos()).collect();
    let mut field = vec![[0.0, 0.0]; WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let this_pos = [x as f64, y as f64];

            let mut closest = cell_points[0];
            let mut closest_distance = distance(this_pos, closest);

            for p in cell_points.iter().cloned() {
                let d = distance(this_pos, p);
                if d < closest_distance {
                    closest = p;
                    closest_distance = d;
                }
            }

            field[x + y * WIDTH] = [this_pos[0] - closest[0], this_pos[1] - closest[1]];
        }
    }

    field
}

struct Point {
    pos: [f64; 2],
    vel: [f64; 2],
    color: [u8; 3],
}

fn random_color() -> [u8; 3] {
    let mut r = js_sys::Math::random();
    let mut g = js_sys::Math::random();
    let mut b = js_sys::Math::random();
    match js_sys::Math::random() {
        x if x < 1.0 / 3.0 => {
            r += (1.0 - r) / 2.0;
        }
        x if x < 2.0 / 3.0 => {
            g = (1.0 - g) / 2.0;
        }
        _ => {
            b = (1.0 - b) / 2.0;
        }
    }
    [
        (r * 2.5).round() as u8,
        (g * 1.9).round() as u8,
        (b * 2.8).round() as u8,
    ]
}

fn random_pos() -> [f64; 2] {
    [
        js_sys::Math::random() * 255.0,
        js_sys::Math::random() * 255.0,
    ]
}

fn random_vel() -> [f64; 2] {
    [
        js_sys::Math::random() * MAX_VELOCITY,
        js_sys::Math::random() * MAX_VELOCITY,
    ]
}

lazy_static! {
    static ref VEC_FIELD: Mutex<Vec<[f64; 2]>> = Mutex::new(vec![]);
    static ref POINTS: Mutex<Vec<Point>> = Mutex::new(
        (0..WIDTH * HEIGHT / 16)
            .map(|_| Point {
                pos: random_pos(),
                vel: random_vel(),
                color: random_color(),
            }).collect()
    );
}

fn clamp_vel(v: f64) -> f64 {
    match v {
        v if v > MAX_VELOCITY => MAX_VELOCITY,
        v if v < -MAX_VELOCITY => -MAX_VELOCITY,
        v => v,
    }
}

#[wasm_bindgen]
pub fn frame(frame_buffer: &mut [u8], key_down: bool) {
    utils::set_panic_hook();

    let mut vec_field = VEC_FIELD.lock().unwrap();
    if key_down {
        vec_field.clear();
    }
    if vec_field.is_empty() {
        vec_field.extend(make_distance_vector_field());

        for (pixel, [x, y]) in frame_buffer.chunks_mut(4).zip(vec_field.iter().cloned()) {
            // The furthest distance possible is about 362 if the pixel is in
            // the opposite corner from every cell center point. Halve that
            // range since we are taking absolute value.
            let x = x.abs() / 181.0;
            let y = y.abs() / 181.0;
            let distance = (x.powi(2) + y.powi(2)).sqrt();
            let delta_xy = (x - y).abs();

            let x = x * 255.0;
            let y = y * 255.0;
            let distance = distance * 255.0;

            pixel[0] = (distance + 2.0 * x * delta_xy) as u8;
            pixel[1] = (distance / 4.0) as u8;
            pixel[2] = (distance + 2.0 * y * delta_xy) as u8;
            pixel[3] = 255;
        }
    }

    let mut points = POINTS.lock().unwrap();
    for p in points.iter_mut() {
        let x = p.pos[0].round() as usize % WIDTH;
        let y = p.pos[1].round() as usize % HEIGHT;

        let v = vec_field[x + y * WIDTH];

        // Don't let points get stuck on the outer edges, which they otherwise
        // tend to do.
        if x == 0 || x == 255 {
            p.pos[0] = js_sys::Math::random() * 255.0;
        }
        if y == 0 || y == 255 {
            p.pos[1] = js_sys::Math::random() * 255.0;
        }

        // Update the point's velocity based on looking up its position in the
        // vector field.
        p.vel[0] += v[0] / 362.0;
        p.vel[0] = clamp_vel(p.vel[0]);
        p.vel[1] += v[1] / 362.0;
        p.vel[1] = clamp_vel(p.vel[1]);

        // Update the point's position based on its current velocity.
        p.pos[0] += p.vel[0];
        p.pos[0] %= WIDTH as f64;
        p.pos[1] += p.vel[1];
        p.pos[1] %= WIDTH as f64;

        // Find the new index for this point within the frame buffer.
        let x = p.pos[0].round() as usize % WIDTH;
        let y = p.pos[1].round() as usize % HEIGHT;
        let idx = (x + y * WIDTH) * 4;

        // Make the color a little lighter where this point is in the frame
        // buffer.
        frame_buffer[idx + 0] = frame_buffer[idx + 0].saturating_add(p.color[0]);
        frame_buffer[idx + 1] = frame_buffer[idx + 1].saturating_add(p.color[1]);
        frame_buffer[idx + 2] = frame_buffer[idx + 2].saturating_add(p.color[2]);
        frame_buffer[idx + 3] = 255;
    }
}
