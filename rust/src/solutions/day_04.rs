use crate::solvable::Solvable;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

pub struct Day04;

impl Solvable for Day04 {
    fn first(&self, input: &str) -> i32 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut count = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                count += count_matches(&mut grid.clone(), r as i32, c as i32);
            }
        }

        count
    }

    fn second(&self, input: &str) -> i32 {
        0
    }
}

fn count_matches(grid: &mut Vec<Vec<char>>, start_row: i32, start_col: i32) -> i32 {
    let mut total_matches = 0;

    for &(dx, dy) in &DIRECTIONS {
        if search_word(grid, 0, start_row, start_col, dx, dy) {
            total_matches += 1;
        }
    }

    total_matches
}

fn search_word(
    grid: &mut Vec<Vec<char>>,
    index: usize,
    row: i32,
    col: i32,
    dx: i32,
    dy: i32,
) -> bool {
    // Base case: all characters matched
    if index == XMAS.len() {
        return true;
    }

    // Check boundaries
    if row < 0 || col < 0 || row >= grid.len() as i32 || col >= grid[0].len() as i32 {
        return false;
    }

    // Check if current cell matches the character at `index`
    if grid[row as usize][col as usize] != XMAS[index] {
        return false;
    }

    // Temporarily mark the cell as visited
    let original_char = grid[row as usize][col as usize];
    grid[row as usize][col as usize] = '#';

    // Move to the next character in the given direction
    let result = search_word(grid, index + 1, row + dx, col + dy, dx, dy);

    // Restore the cell's original state
    grid[row as usize][col as usize] = original_char;

    result
}
