use crate::solvable::Solvable;

struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let parts = value.split(':').collect::<Vec<_>>();
        let result = parts[0].trim().parse::<i64>().unwrap();
        let numbers = parts[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        Self { result, numbers }
    }
}

pub struct Day07;

impl Solvable for Day07 {
    fn first(&self, input: &str) -> i64 {
        let equations = input
            .lines()
            .map(|line| Equation::from(line))
            .collect::<Vec<_>>();

        let mut total_calibration_result = 0;

        for equation in equations {
            if self.is_solvable(&equation) {
                total_calibration_result += equation.result;
            }
        }

        total_calibration_result
    }

    fn second(&self, input: &str) -> i64 {
        let equations = input
            .lines()
            .map(|line| Equation::from(line))
            .collect::<Vec<_>>();

        let mut total_calibration_result = 0;

        for equation in equations {
            if self.is_solvable_with_concat(&equation) {
                total_calibration_result += equation.result;
            }
        }

        total_calibration_result
    }
}

impl Day07 {
    fn is_solvable(&self, equation: &Equation) -> bool {
        let numbers = &equation.numbers;
        let target = equation.result;

        let num_count = numbers.len();

        if num_count < 2 {
            return false;
        }

        let operator_combinations = 2usize.pow((num_count - 1) as u32);

        for combination in 0..operator_combinations {
            if self.evaluate_combination(numbers, combination) == target {
                return true;
            }
        }

        false
    }

    fn is_solvable_with_concat(&self, equation: &Equation) -> bool {
        let numbers = &equation.numbers;
        let target = equation.result;

        let num_count = numbers.len();
        if num_count < 2 {
            return false; // Not enough numbers to form an equation
        }

        let operator_combinations = 3usize.pow((num_count - 1) as u32); // 3^(n-1) combinations of +, *, ||

        for combination in 0..operator_combinations {
            if self.evaluate_combination_with_concat(numbers, combination) == target {
                return true;
            }
        }

        false
    }

    fn evaluate_combination(&self, numbers: &[i64], combination: usize) -> i64 {
        let mut result = numbers[0];

        for (i, &num) in numbers.iter().enumerate().skip(1) {
            let operation = (combination >> (i - 1)) & 1;
            if operation == 0 {
                result += num;
            } else {
                result *= num;
            }
        }

        result
    }

    fn evaluate_combination_with_concat(&self, numbers: &[i64], combination: usize) -> i64 {
        let mut result = numbers[0];
        let mut current_combination = combination;

        for &num in &numbers[1..] {
            let operation = current_combination % 3;
            current_combination /= 3;

            match operation {
                0 => result += num,
                1 => result *= num,
                2 => result = self.concat(result, num),
                _ => unreachable!(),
            }
        }

        result
    }

    fn concat(&self, left: i64, right: i64) -> i64 {
        let mut left = left.to_string();
        let right = right.to_string();
        left.push_str(&right);
        left.parse::<i64>().unwrap()
    }
}
