use std::num::NonZeroU8;

mod candidates;
mod load;
mod solve;
mod types;

fn main() {
    let mut test_puzzle: types::Puzzle = [[None; 9]; 9];
    "
800000000
003600000
070090200
050007000
000045700
000100030
001000068
008500010
090000400
    "
    .trim()
    .lines()
    .enumerate()
    .flat_map(|(y, row)| {
        row.chars()
            .enumerate()
            .map(move |(x, cell)| (x, y, cell.to_digit(10).unwrap() as u8))
    })
    .for_each(|(x, y, cell)| {
        test_puzzle[y][x] = NonZeroU8::new(cell);
    });
    let mut test_grid = load::generate_cells(test_puzzle);
    let test_subs = load::generate_subsections(&test_grid);
    // let test_candidates = candidates::get_candidates((0, 1), &test_subs, false);
    candidates::generate_candidates(&mut test_grid, false);
    println!("{:?}", test_grid);
}
