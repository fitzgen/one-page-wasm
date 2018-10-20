extern crate cfg_if;
extern crate lazy_static;
extern crate wasm_bindgen;

mod utils;

use lazy_static::lazy_static;
use std::f64;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

const NUM_BOIDS: usize = 25;

struct State {
    flock: Vec<Boid>,
    first_frame: bool,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {
        flock: (0..NUM_BOIDS)
            .zip(0..NUM_BOIDS)
            .map(|(x, y)| {
                let x = x as f64;
                let y = y as f64;
                Boid {
                    position: [x * 10.0 % WIDTH as f64, (y * 25.0 + 50.0) % HEIGHT as f64],
                    direction: x * y,
                    color: COLORS[x as usize % COLORS.len()],
                }
            }).collect(),
        first_frame: true,
    });
}

#[derive(Copy, Clone)]
struct Boid {
    position: [f64; 2],
    direction: f64,
    color: Color,
}

impl Boid {
    const RADIUS: f64 = 30.0;

    fn draw(&self, buf: &mut [u8]) {
        let velocity = [self.direction.sin(), self.direction.cos()];
        let x = self.position[0];
        let y = self.position[1];
        for i in 0..1_i32 {
            let i = i as f64;
            set_pixel(
                buf,
                (x + velocity[0] * i) as usize,
                (y + velocity[1] * i) as usize,
                self.color,
            );
        }
    }

    fn next(&self, me: usize, flock: &[Boid]) -> Boid {
        let mut closest = None;

        let mut sum_dir = self.direction;
        let mut sum_pos = self.position;
        let mut num_near = 1_u32;

        flock
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != me)
            .for_each(|(_, b)| {
                let b_dist_sq = self.distance_squared(b);

                if b_dist_sq <= Self::RADIUS * Self::RADIUS {
                    num_near += 1;
                    sum_dir += b.direction;
                    sum_pos[0] += b.position[0];
                    sum_pos[1] += b.position[1];
                }

                if b_dist_sq > 50.0 {
                    return;
                }

                closest = Some(closest.map_or(b, |c| {
                    if b_dist_sq < self.distance_squared(c) {
                        b
                    } else {
                        c
                    }
                }));
            });

        let avg_pos = [sum_pos[0] / num_near as f64, sum_pos[1] / num_near as f64];
        let avg_dir = sum_dir / num_near as f64;

        let mut next = self.clone();

        let velocity = [self.direction.sin(), self.direction.cos()];
        let left = [-velocity[1], velocity[0]];

        if num_near > 1 {
            let delta = [avg_dir.sin(), avg_dir.cos()];
            let left_right = left[0] * delta[0] + left[1] * delta[1];
            next.direction = if left_right > 0.0 {
                next.direction - 0.25 * left_right
            } else {
                next.direction - 0.25 * left_right
            };

            let delta = [avg_pos[0] - self.position[0], avg_pos[1] - self.position[1]];
            let left_right = left[0] * delta[0] + left[1] * delta[1];
            next.direction = if left_right > 0.0 {
                next.direction - 0.02 * left_right
            } else {
                next.direction - 0.02 * left_right
            };
        }

        if let Some(closest) = closest {
            let delta = [
                closest.position[0] - self.position[0],
                closest.position[1] - self.position[1],
            ];
            let left_right = left[0] * delta[0] + left[1] * delta[1];
            next.direction = if left_right > 0.0 {
                next.direction + 0.2
            } else {
                next.direction - 0.2
            };
        }

        next.position = [
            (self.position[0] + velocity[0] + WIDTH as f64) % WIDTH as f64,
            (self.position[1] + velocity[1] + HEIGHT as f64) % HEIGHT as f64,
        ];
        next
    }

    fn distance_squared(&self, b: &Boid) -> f64 {
        let dx = self.position[0] - b.position[0];
        let dy = self.position[1] - b.position[1];
        dx * dx + dy * dy
    }
}

#[derive(Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const COLORS: &[Color] = &[
    Color {
        r: 200,
        g: 60,
        b: 80,
        a: 255,
    },
    Color {
        r: 30,
        g: 80,
        b: 90,
        a: 255,
    },
    Color {
        r: 70,
        g: 149,
        b: 100,
        a: 255,
    },
    Color {
        r: 12,
        g: 120,
        b: 200,
        a: 255,
    },
    Color {
        r: 10,
        g: 80,
        b: 75,
        a: 255,
    },
    Color {
        r: 80,
        g: 10,
        b: 65,
        a: 255,
    },
    Color {
        r: 90,
        g: 65,
        b: 5,
        a: 255,
    },
    Color {
        r: 230,
        g: 10,
        b: 10,
        a: 255,
    },
    Color {
        r: 10,
        g: 230,
        b: 10,
        a: 255,
    },
    Color {
        r: 10,
        g: 10,
        b: 230,
        a: 255,
    },
];

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn set_pixel(buf: &mut [u8], x: usize, y: usize, color: Color) {
    assert!(buf.len() as usize == WIDTH * HEIGHT * 4);
    if x >= WIDTH || y >= HEIGHT {
        return;
    }
    let a = (color.a as f64) / 255.0;
    let idx = x * 4 + y * WIDTH * 4;
    buf[idx + 0] = (buf[idx + 0] as f64 * (1.0 - a) + color.r as f64 * a).ceil() as u8;
    buf[idx + 1] = (buf[idx + 1] as f64 * (1.0 - a) + color.g as f64 * a).ceil() as u8;
    buf[idx + 2] = (buf[idx + 2] as f64 * (1.0 - a) + color.b as f64 * a).ceil() as u8;
    buf[idx + 3] = 255;
}

#[wasm_bindgen]
pub fn frame(frame_buffer: &mut [u8], key_down: bool) {
    utils::set_panic_hook();

    let mut state = STATE.lock().unwrap();

    let bg_alpha = if state.first_frame { 255 } else { 40 };
    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            set_pixel(
                frame_buffer,
                x,
                y,
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: bg_alpha,
                },
            );
        }
    }

    if key_down {
        let n = state.flock.len();
        state.flock.push(Boid {
            position: [128.0, 128.0],
            direction: 0.0,
            color: COLORS[n % COLORS.len()],
        });
    }

    let new_flock: Vec<_> = state
        .flock
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.draw(frame_buffer);
            b.next(i, &state.flock)
        }).collect();

    state.flock = new_flock;
    state.first_frame = false;
}
