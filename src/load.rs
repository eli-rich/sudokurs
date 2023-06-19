use std::array;

use crate::types::*;

pub fn generate_cells(puzzle: Puzzle) -> Grid {
    let grid: Grid = array::from_fn(|y| {
        array::from_fn(|x| {
            let position = (x as u8, y as u8);
            let value = puzzle[y][x];
            let candidates = match value {
                Some(_value) => None,
                None => Some(vec![]),
            };
            Cell {
                value,
                candidates,
                position,
            }
        })
    });
    grid
}

pub fn generate_subsections(grid: &Grid) -> Subsections {
    let rows: [[Cell; 9]; 9] = array::from_fn(|y| array::from_fn(|x| grid[y][x].clone()));
    let columns: [[Cell; 9]; 9] = array::from_fn(|x| array::from_fn(|y| grid[y][x].clone()));
    let boxes: [[Cell; 9]; 9] = array::from_fn(|box_y| {
        array::from_fn(|box_x| {
            grid[3 * (box_y / 3) + (box_x / 3)][3 * (box_y % 3) + (box_x % 3)].clone()
        })
    });
    Subsections {
        rows,
        columns,
        boxes,
    }
}
