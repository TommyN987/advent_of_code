use crate::solvable::Solvable;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const DIRECTIONS: [(i64, i64); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

const TOP_MAS_PATTERN: [[(char, usize, usize); 3]; 2] = [
    // Forward MAS
    [('M', 0, 0), ('A', 1, 1), ('S', 2, 2)],
    // Reversed MAS
    [('S', 0, 0), ('A', 1, 1), ('M', 2, 2)],
];

const BOTTOM_MAS_PATTERN: [[(char, usize, usize); 3]; 2] = [
    // Forward MAS
    [('M', 2, 0), ('A', 1, 1), ('S', 0, 2)],
    // Reversed MAS
    [('S', 2, 0), ('A', 1, 1), ('M', 0, 2)],
];

pub struct Day04;

impl Solvable for Day04 {
    fn first(&self, input: &str) -> i64 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut count = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                count += count_matches(&grid, r as i64, c as i64);
            }
        }

        count
    }

    fn second(&self, input: &str) -> i64 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut count = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..(rows - 2) {
            for c in 0..(cols - 2) {
                if is_xmas(&grid, r, c) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn count_matches(grid: &Vec<Vec<char>>, start_row: i64, start_col: i64) -> i64 {
    let mut total_matches = 0;

    for &(dx, dy) in &DIRECTIONS {
        if search_word(grid, 0, start_row, start_col, dx, dy) {
            total_matches += 1;
        }
    }

    total_matches
}

fn search_word(grid: &Vec<Vec<char>>, index: usize, row: i64, col: i64, dx: i64, dy: i64) -> bool {
    // Base case: all characters matched
    if index == XMAS.len() {
        return true;
    }

    // Check boundaries
    if row < 0 || col < 0 || row >= grid.len() as i64 || col >= grid[0].len() as i64 {
        return false;
    }

    // Check if current cell matches the character at `index`
    if grid[row as usize][col as usize] != XMAS[index] {
        return false;
    }

    // Move to the next character in the given direction
    let result = search_word(grid, index + 1, row + dx, col + dy, dx, dy);

    result
}

fn is_xmas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    for top_pattern in &TOP_MAS_PATTERN {
        for bottom_pattern in &BOTTOM_MAS_PATTERN {
            if top_pattern
                .iter()
                .all(|&(ch, dr, dc)| grid[row + dr][col + dc] == ch)
                && bottom_pattern
                    .iter()
                    .all(|&(ch, dr, dc)| grid[row + dr][col + dc] == ch)
            {
                return true;
            }
        }
    }

    false
}
