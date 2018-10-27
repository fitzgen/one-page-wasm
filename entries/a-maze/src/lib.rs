extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
use std::ops::{Add, Div, Mul};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

const WIDTH: isize = 256;
const HEIGHT: isize = 256;
const GRID: isize = 15; // odd, divisible by 255
const DOT_SIZE: isize = 10; // even, less than GRID
const CELL_DIM: isize = WIDTH / GRID;

const HEAD_COLOR: Color = Color {
    r: 143,
    g: 59,
    b: 27,
};
const TAIL_COLOR: Color = Color {
    r: 185,
    g: 156,
    b: 107,
};
const BORDER_COLOR: Color = Color {
    r: 73,
    g: 56,
    b: 41,
};
const VISITED_COLOR: Color = Color {
    r: 189,
    g: 208,
    b: 156,
};
const UNVISITED_COLOR: Color = Color {
    r: 102,
    g: 141,
    b: 60,
};

const CONNECTED: Color = VISITED_COLOR;

#[derive(PartialEq, Copy, Clone)]
enum CellType {
    OutOfBounds,
    Head,
    Tail,
    Visited,
    Unvisited,
}

#[derive(PartialEq, Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<isize> for Pos {
    type Output = Pos;

    fn add(self, other: isize) -> Pos {
        Pos {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Mul<isize> for Pos {
    type Output = Pos;

    fn mul(self, other: isize) -> Pos {
        Pos {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<isize> for Pos {
    type Output = Pos;

    fn div(self, other: isize) -> Pos {
        Pos {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Pos {
    fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }
}

fn set_pixel(frame_buffer: &mut [u8], pos: Pos, color: Color) {
    let idx = (pos.x + pos.y * WIDTH) as usize * 4;
    frame_buffer[idx + 0] = color.r;
    frame_buffer[idx + 1] = color.g;
    frame_buffer[idx + 2] = color.b;
}

fn get_pixel(frame_buffer: &[u8], pos: Pos) -> Color {
    let idx = (pos.x + pos.y * WIDTH) as usize * 4;
    Color {
        r: frame_buffer[idx + 0],
        g: frame_buffer[idx + 1],
        b: frame_buffer[idx + 2],
    }
}

fn full_square(frame_buffer: &mut [u8], coord: Pos, color: Color) {
    let corner = coord * GRID + 1;

    for dx in 0..(GRID - 1) {
        for dy in 0..(GRID - 1) {
            set_pixel(frame_buffer, corner + Pos::new(dx, dy), color);
        }
    }
}

fn head_pos(coord: Pos) -> Pos {
    coord * GRID + (GRID - DOT_SIZE + 1) / 2
}

fn middle_square(frame_buffer: &mut [u8], coord: Pos, color: Color) {
    let pos = head_pos(coord);
    for dx in 0..DOT_SIZE {
        for dy in 0..DOT_SIZE {
            set_pixel(frame_buffer, pos + Pos::new(dx, dy), color);
        }
    }
}

fn set_head(frame_buffer: &mut [u8], coord: Pos) {
    middle_square(frame_buffer, coord, HEAD_COLOR);
}

fn set_tail(frame_buffer: &mut [u8], coord: Pos) {
    middle_square(frame_buffer, coord, TAIL_COLOR);
}

fn init_buffer(frame_buffer: &mut [u8]) {
    for (y, row) in frame_buffer.chunks_mut(WIDTH as usize * 4).enumerate() {
        for (x, chunk) in row.chunks_mut(4).enumerate() {
            assert!(chunk.len() == 4);
            let is_edge = x as isize % GRID == 0 || y as isize % GRID == 0;
            let color = if is_edge {
                BORDER_COLOR
            } else {
                UNVISITED_COLOR
            };
            chunk[0] = color.r;
            chunk[1] = color.g;
            chunk[2] = color.b;
            chunk[3] = 255;
        }
    }

    let mid = CELL_DIM / 2;
    set_head(frame_buffer, Pos::new(mid, mid));
}

fn find_head(frame_buffer: &[u8]) -> Option<Pos> {
    for y in 0..CELL_DIM {
        for x in 0..CELL_DIM {
            let coord = Pos::new(x, y);
            if cell_type(frame_buffer, coord) == CellType::Head {
                return Some(coord);
            }
        }
    }

    None
}

fn cell_type(frame_buffer: &[u8], coord: Pos) -> CellType {
    if coord.x < 0 || coord.y < 0 || coord.x >= CELL_DIM || coord.y >= CELL_DIM {
        return CellType::OutOfBounds;
    }

    let head_pixel = get_pixel(frame_buffer, head_pos(coord));
    if head_pixel == HEAD_COLOR {
        return CellType::Head;
    }
    if head_pixel == TAIL_COLOR {
        return CellType::Tail;
    }
    if head_pixel == VISITED_COLOR {
        return CellType::Visited;
    }
    if head_pixel == UNVISITED_COLOR {
        return CellType::Unvisited;
    }

    return CellType::OutOfBounds;
}

// Find the middle coordinate between two neighboring cells
fn mid(from: Pos, to: Pos) -> Pos {
    assert!((from.x - to.x).abs() + (from.y - to.y).abs() == 1);

    ((from + to + 1) * GRID) / 2
}

fn connect(frame_buffer: &mut [u8], from: Pos, to: Pos) {
    let dx = from.x - to.x;
    let dy = from.y - to.y;
    assert!((dx).abs() + (dy).abs() == 1);
    let wall_dir = Pos::new(dy.abs(), dx.abs());

    let mid = mid(from, to);
    for i in 0..(GRID - 1) {
        let pos = mid + wall_dir * (i as isize - GRID / 2 + 1);
        set_pixel(frame_buffer, pos, CONNECTED);
    }
}

fn is_connected(frame_buffer: &[u8], from: Pos, to: Pos) -> bool {
    let mid = mid(from, to);
    get_pixel(frame_buffer, mid) != BORDER_COLOR
}

#[wasm_bindgen]
pub fn frame(frame_buffer: &mut [u8], key_down: bool) {
    utils::set_panic_hook();

    assert!(frame_buffer.len() == (WIDTH * HEIGHT * 4) as usize);
    if frame_buffer[3] != 255 || key_down {
        init_buffer(frame_buffer);
    } else {
        let head_coord = find_head(frame_buffer);

        let mut directions: [Pos; 4] = [
            Pos::new(0, -1),
            Pos::new(-1, 0),
            Pos::new(0, 1),
            Pos::new(1, 0),
        ];

        if let Some(head_coord) = head_coord {
            let mut advanced = false;
            // shuffle
            for i in 0..directions.len() {
                let j = (random() * (directions.len() - i) as f64) as usize;
                directions.swap(i, j);
            }

            for dir in directions.iter() {
                let new_coord = head_coord + *dir;
                if cell_type(frame_buffer, new_coord) == CellType::Unvisited {
                    full_square(frame_buffer, head_coord, VISITED_COLOR);
                    set_tail(frame_buffer, head_coord);
                    set_head(frame_buffer, new_coord);
                    connect(frame_buffer, head_coord, new_coord);
                    advanced = true;
                    break;
                }
            }
            if !advanced {
                // backtrack
                full_square(frame_buffer, head_coord, VISITED_COLOR);
                for dir in directions.iter() {
                    let backtrack_coord = head_coord + *dir;
                    if cell_type(frame_buffer, backtrack_coord) == CellType::Tail
                        && is_connected(frame_buffer, head_coord, backtrack_coord)
                    {
                        set_head(frame_buffer, backtrack_coord);
                        return;
                    }
                }
            }
        }
    }
}
