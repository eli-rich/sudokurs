use std::num::NonZeroU8;

pub type Position = (u8, u8);

#[derive(Clone, Debug)]
pub struct Cell {
    pub value: Option<NonZeroU8>,
    pub candidates: Option<Vec<u8>>,
    pub position: Position,
}

pub type Puzzle = [[Option<NonZeroU8>; 9]; 9];

pub type Row = [Cell; 9];
pub type Column = [Cell; 9];
pub type Box = [Cell; 9];

pub type Grid = [[Cell; 9]; 9];

pub struct Subsections {
    pub rows: [[Cell; 9]; 9],
    pub columns: [[Cell; 9]; 9],
    pub boxes: [[Cell; 9]; 9],
}
