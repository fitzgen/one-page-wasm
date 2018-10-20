extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;
use wasm_bindgen::prelude::*;

static mut TIME: u32 = 0;

#[wasm_bindgen]
pub fn frame(frame_buffer: &mut [u8], _key_down: bool) {
    utils::set_panic_hook();

    let time = unsafe {
        let t = TIME;
        TIME += 1;
        t
    };

    for (y, row) in frame_buffer.chunks_mut(256 * 4).enumerate() {
        for (x, chunk) in row.chunks_mut(4).enumerate() {
            assert!(chunk.len() == 4);
            let r = ( ( f64::from(time)  / 100.0 ).sin() * 128.0 + 128.0 ) as u8;
            let g = ( ( f64::from(time)  / 10.0 ).cos() * 128.0 + 128.0 ) as u8;
            let b = ( ( f64::from(time + x as u32 + y as u32)  / 50.0 ).cos() * 128.0 + 128.0 ) as u8;
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = 255;
        }
    }
}
