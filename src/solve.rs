use std::collections::HashMap;
use std::num::NonZeroU8;

use crate::candidates::generate_candidates;
use crate::types::{Grid, Position};

fn is_valid_candidate(grid: &Grid, pos: &Position, candidate: u8) -> bool {
    let (row, column) = pos;
    let mut valid = true;
    for i in 0..9 {
        if grid[*row as usize][i].value == NonZeroU8::new(candidate) {
            valid = false;
            break;
        }
        if grid[i][*column as usize].value == NonZeroU8::new(candidate) {
            valid = false;
            break;
        }
    }
    if valid {
        let box_row = row / 3;
        let box_column = column / 3;
        for i in 0..3 {
            for j in 0..3 {
                if grid[3 * box_row as usize + i][3 * box_column as usize + j].value
                    == (NonZeroU8::new(candidate))
                {
                    valid = false;
                    break;
                }
            }
        }
    }
    valid
}

pub fn solve(mut grid: Grid, verify: bool, generating: bool) -> bool {
    let mut empty_cells = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j].value == None {
                empty_cells.push((i, j));
            }
        }
    }
    let mut index = 0;
    let mut solutions = 0;

    generate_candidates(&mut grid, generating);
    let mut should_try: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut initials: HashMap<u8, Vec<u8>> = HashMap::new();

    for i in 0..empty_cells.len() {
        should_try.insert(
            i as u8,
            grid[empty_cells[i].0][empty_cells[i].1]
                .candidates
                .clone()
                .unwrap(),
        );
        initials.insert(
            i as u8,
            grid[empty_cells[i].0][empty_cells[i].1]
                .candidates
                .clone()
                .unwrap(),
        );
    }

    while index < empty_cells.len() {
        if verify && index < 0 {
            return solutions == 1;
        }
        let (row, column) = empty_cells[index];
        let mut cell = grid[row][column];
        let mut candidates = should_try.get_mut(&(index as u8)).unwrap();
        let mut candidate = candidates.pop();

        if candidate == None {
            cell.value = None;
            index -= 1;
            continue;
        } else {
            if !is_valid_candidate(&grid, &(row as u8, column as u8), candidate.unwrap()) {
                continue;
            }
            cell.clone().value = NonZeroU8::new(candidate.unwrap());
            generate_candidates(&mut grid, generating);
            index += 1;
        }
    }

    true
}
