use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// Important to have this so that each cell is represented as a single
// byte
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    DEAD = 0,
    ALIVE = 1
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::DEAD => Cell::ALIVE,
            Cell::ALIVE => Cell::DEAD,
        };
    }
}
