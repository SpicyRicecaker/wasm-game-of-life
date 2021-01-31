mod utils;

use std::borrow::Borrow;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

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

impl Universe {
    fn new(width: u32, height: u32) -> Self {
        let mut cells = Vec::new();
        for _ in 0..width * height {
            cells.push(Cell::Dead)
        }

        Universe {
            width,
            height,
            cells,
        }
    }
    // Returns the index of the cell in terms of wasm linear memory
    fn get_index(&self, row: u32, column: u32) -> usize {
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
