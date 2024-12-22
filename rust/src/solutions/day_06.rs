use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use crate::solvable::Solvable;

pub struct Day06;

#[derive(Clone)]
enum Cell {
    Guard(Direction),
    Empty,
    Visited,
    Obstacle,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Obstacle,
            '^' => Cell::Guard(Direction::North),
            _ => Cell::Empty,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn next_position(self, row: usize, col: usize) -> (isize, isize) {
        match self {
            Direction::North => (row as isize - 1, col as isize),
            Direction::South => (row as isize + 1, col as isize),
            Direction::East => (row as isize, col as isize + 1),
            Direction::West => (row as isize, col as isize - 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(value: Position) -> Self {
        (value.x, value.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GuardState {
    position: Position,
    direction: Direction,
}

#[derive(Clone)]
struct GridState {
    grid: Vec<Vec<Cell>>,
    guard_pos: Position,
    guard_dir: Direction,
    visited: HashSet<Position>,
}

impl Solvable for Day06 {
    fn first(&self, input: &str) -> i32 {
        let mut grid_state = self.process_input(input);
        self.simulate_patrol(&mut grid_state) as i32
    }

    fn second(&self, input: &str) -> i32 {
        let mut grid_state = self.process_input(input);
        self.find_loop_positions(&mut grid_state) as i32
    }
}

impl Day06 {
    fn process_input(&self, input: &str) -> GridState {
        let mut guard_pos = Position::default();
        let mut visited = HashSet::new();
        let mut guard_dir = Direction::North;
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, ch)| {
                        let cell = Cell::from(ch);
                        if let Cell::Guard(dir) = &cell {
                            guard_pos = (i, j).into();
                            visited.insert(guard_pos);
                            guard_dir = *dir;
                        }
                        cell
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        GridState {
            grid,
            visited,
            guard_pos,
            guard_dir,
        }
    }

    fn simulate_patrol(
        &self,
        GridState {
            grid,
            guard_pos,
            guard_dir,
            visited,
        }: &mut GridState,
    ) -> usize {
        loop {
            let (next_row, next_col) = guard_dir.next_position(guard_pos.x, guard_pos.y);

            if next_row < 0
                || next_col < 0
                || next_row as usize >= grid.len()
                || next_col as usize >= grid[0].len()
            {
                break;
            }

            let (next_row, next_col) = (next_row as usize, next_col as usize);

            match grid[next_row][next_col] {
                Cell::Obstacle => {
                    *guard_dir = guard_dir.turn_right();
                }
                _ => {
                    *guard_pos = Position::from((next_row, next_col));
                    if let Cell::Empty = grid[next_row][next_col] {
                        grid[next_row][next_col] = Cell::Visited;
                        visited.insert(*guard_pos);
                    }
                }
            }
        }
        visited.len()
    }

    fn find_loop_positions(&self, grid_state: &mut GridState) -> usize {
        let now = Instant::now();
        let mut loop_positions = HashSet::new();

        // Simulate patrol
        let mut visited_states = HashSet::new();
        let mut current_state = GuardState {
            position: grid_state.guard_pos,
            direction: grid_state.guard_dir,
        };

        loop {
            let (row, col) = (current_state.position.x, current_state.position.y);
            let dir = current_state.direction;

            // If we've already visited this state, break to prevent an infinite loop
            if !visited_states.insert(current_state) {
                break;
            }

            // Attempt to place an obstruction at each valid position
            if let Cell::Empty = grid_state.grid[row][col] {
                let mut test_state = grid_state.clone();
                test_state.grid[row][col] = Cell::Obstacle;

                // Check if the guard gets stuck in a loop
                let mut test_visited_states = HashSet::new();
                let mut test_current_state = GuardState {
                    position: test_state.guard_pos,
                    direction: test_state.guard_dir,
                };
                let mut is_loop = false;

                loop {
                    let (test_row, test_col) =
                        (test_current_state.position.x, test_current_state.position.y);
                    let test_dir = test_current_state.direction;

                    // Check bounds
                    let (next_row, next_col) = test_dir.next_position(test_row, test_col);
                    if next_row < 0
                        || next_col < 0
                        || next_row as usize >= test_state.grid.len()
                        || next_col as usize >= test_state.grid[0].len()
                    {
                        break;
                    }

                    let (next_row, next_col) = (next_row as usize, next_col as usize);

                    // Check if the state is revisited
                    if !test_visited_states.insert(test_current_state) {
                        is_loop = true;
                        break;
                    }

                    match test_state.grid[next_row][next_col] {
                        Cell::Obstacle => {
                            test_current_state.direction =
                                test_current_state.direction.turn_right();
                        }
                        _ => {
                            test_current_state.position = Position::from((next_row, next_col));
                        }
                    }
                }

                if is_loop {
                    loop_positions.insert(current_state.position);
                }
            }

            // Move the guard to the next state in the original grid
            let (next_row, next_col) = dir.next_position(row, col);

            if next_row < 0
                || next_col < 0
                || next_row as usize >= grid_state.grid.len()
                || next_col as usize >= grid_state.grid[0].len()
            {
                break;
            }

            let (next_row, next_col) = (next_row as usize, next_col as usize);

            match grid_state.grid[next_row][next_col] {
                Cell::Obstacle => {
                    current_state.direction = current_state.direction.turn_right();
                }
                _ => {
                    current_state.position = Position::from((next_row, next_col));
                }
            }
        }
        println!("{:?}", now.elapsed().as_millis());
        loop_positions.len()
    }
}
