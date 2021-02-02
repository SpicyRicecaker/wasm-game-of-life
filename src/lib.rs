mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello {}, how are you doing today?", name));
// }

#[wasm_bindgen]
// Specifies that each cell is represented as a single byte
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    // Dead is `0`, alive is `1`, we can easily count neighbors with addition
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen]
    pub fn new(width: u32, height: u32) -> Self {
        // let mut cells = Vec::new();
        // for _ in 0..width * height {
        //     cells.push(Cell::Dead)
        // }
        // let width = 64;
        // let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    // Returns the index of the cell in terms of wasm linear memory
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        // Multiply row by width and add column
        (row * self.width + column) as usize
    }

    // Get count of live neighbors of a cell
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut neighbors = 0;
        for dy in -1..2 {
            for dx in -1..2 {
                // Dont' do anything in the case of the square itself
                if dy == 0 && dx == 0 {
                    continue;
                }
                neighbors += match self.cells[self.get_index(
                    (row as i32 + dy + self.height as i32) as u32 % self.height,
                    (column as i32 + dx + self.width as i32) as u32 % self.width,
                )] {
                    Cell::Dead => 0,
                    Cell::Alive => 1,
                }
            }
        }

        neighbors
    }

    // Each game round
    pub fn tick(&mut self) {
        // Clone all cells so we can easily update the game board
        let mut next = self.cells.clone();

        // Loop through every cell in the board
        for row in 0..self.width {
            for column in 0..self.height {
                // Get the actual index of this cell
                let index = self.get_index(row, column);
                let state = self.cells[index];

                // Set this cell depending on the state of the cell and surrounding neighbors,
                // According to Conway's game of life
                // I fkin love rust enums & matching wtf
                next[index] = match (state, self.live_neighbor_count(row, column)) {
                    // if alive,
                    // with 2-3 neighbors, you live
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // otherwise, you die
                    (Cell::Alive, _) => Cell::Dead,
                    // if dead,
                    // width 3 neighbors, you are born
                    (Cell::Dead, 3) => Cell::Alive,
                    // otherwise, you ded
                    _ => Cell::Dead,
                }
            }
        }
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
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
    
    pub fn toggle_cell(&mut self, idx: usize) {
        let new_state = match self.cells[idx] {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead
        };
        self.cells[idx] = new_state;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    // Tests number of neighbors given cross
    fn neighbors_full_test() {
        let mut universe = Universe::new(3, 3);

        // Fill up every cell
        let mut indices: Vec<usize> = Vec::new();

        for y in 0..universe.height {
            for x in 0..universe.width {
                indices.push(universe.get_index(y, x));
            }
        }

        for idx in indices.iter() {
            universe.cells[*idx] = Cell::Alive;
        }
        // Fill up every cell

        // Bottom right
        assert_eq!(universe.live_neighbor_count(2, 2), 8);
        // Middle
        assert_eq!(universe.live_neighbor_count(1, 1), 8);
        // Top left
        assert_eq!(universe.live_neighbor_count(0, 0), 8);
        // Top right
        assert_eq!(universe.live_neighbor_count(0, 2), 8);
        // Bot left
        assert_eq!(universe.live_neighbor_count(2, 0), 8);
    }
}
