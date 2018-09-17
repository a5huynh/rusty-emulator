extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use std::fmt;
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

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    // Translate a (row, col) into an index into a flat array.
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // Row above, present row, and the row below.
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8
            }
        }

        count
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::ALIVE
            } else {
                Cell::DEAD
            }
        }).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match(cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbors
                    // dies, as if caused by underpopulation.
                    (Cell::ALIVE, x) if x < 2 => Cell::DEAD,
                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generation.
                    (Cell::ALIVE, 2) | (Cell::ALIVE, 3) => Cell::ALIVE,
                    // Rule 3: Any live cell with more than three live neighbors
                    // dies as if by overpopulation.
                    (Cell::ALIVE, x) if x > 3 => Cell::DEAD,
                    // Rule 4: Any dead cell with exactly three live neighbors
                    // becomes a live cell, as if by reproduction.
                    (Cell::DEAD, 3) => Cell::ALIVE,
                    // All other cells remain in the same state
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::DEAD { '◻' } else { '◼' };
                write!(f, "{}", symbol)?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}