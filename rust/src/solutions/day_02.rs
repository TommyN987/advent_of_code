use crate::solvable::Solvable;

#[derive(PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

impl Safety {
    fn parse(input: &[i32]) -> Self {
        let is_increasing = input.windows(2).all(|pair| pair[1] > pair[0]);
        let is_decreasing = input.windows(2).all(|pair| pair[1] < pair[0]);
        let valid_differences = input
            .windows(2)
            .all(|pair| (pair[1] - pair[0]).abs() >= 1 && (pair[1] - pair[0]).abs() <= 3);

        if (is_increasing || is_decreasing) && valid_differences {
            Self::Safe
        } else {
            Self::Unsafe
        }
    }
}

pub struct Day02;

impl Solvable for Day02 {
    fn first(&self, input: &str) -> i32 {
        let lines = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|char| char.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();

        let safe_lines: Vec<Vec<i32>> = lines
            .into_iter()
            .filter(|line| Safety::parse(line) == Safety::Safe)
            .collect();

        safe_lines.len() as i32
    }

    fn second(&self, input: &str) -> i32 {
        let lines = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|char| char.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();

        let safe_lines_count = lines
            .into_iter()
            .filter(|line| {
                if Safety::parse(line) == Safety::Safe {
                    return true;
                }

                for i in 0..line.len() {
                    let mut modified_line = line.clone();
                    modified_line.remove(i);
                    if Safety::parse(&modified_line) == Safety::Safe {
                        return true;
                    }
                }

                false // No safe version found
            })
            .count();

        safe_lines_count as i32
    }
}
