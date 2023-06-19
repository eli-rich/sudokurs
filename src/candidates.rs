use std::num::NonZeroU8;

use crate::types::*;

use crate::load::generate_subsections;

pub fn get_candidates(pos: Position, subssections: &Subsections, generating: bool) -> Vec<u8> {
    let mut candidates: Vec<u8> = (1..=9).collect();

    let (row, column) = pos;

    let cellRow = &subssections.rows[row as usize];
    let cellColumn = &subssections.columns[column as usize];
    let cellBox = &subssections.boxes[3 * (row / 3) as usize + (column / 3) as usize];

    for cell in cellRow {
        if let Some(value) = cell.value {
            candidates.retain(|&candidate| candidate != value.get());
        }
    }

    for cell in cellColumn {
        if let Some(value) = cell.value {
            candidates.retain(|&candidate| candidate != value.get());
        }
    }

    for cell in cellBox {
        if let Some(value) = cell.value {
            candidates.retain(|&candidate| candidate != value.get());
        }
    }
    candidates
}

pub fn generate_candidates(grid: &mut Grid, generating: bool) -> () {
    let subsections = generate_subsections(&grid);
    for i in 0..9 {
        for j in 0..9 {
            let mut cell = &mut grid[i][j];
            if cell.value == None {
                let candidates = get_candidates((i as u8, j as u8), &subsections, generating);
                cell.candidates = Some(candidates);
            }
        }
    }
}
