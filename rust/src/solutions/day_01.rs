use std::collections::{HashMap, HashSet};

use crate::solvable::Solvable;

pub struct Day01;

impl Solvable for Day01 {
    fn first(&self, input: &str) -> i32 {
        let input = input
            .lines()
            .filter_map(|line| {
                if let Some((left, right)) = line.split_once(char::is_whitespace) {
                    Some((left.trim().to_string(), right.trim().to_string()))
                } else {
                    None
                }
            })
            .collect::<(Vec<String>, Vec<String>)>();
        let (mut left, mut right) = input;

        left.sort();
        right.sort();

        left.into_iter()
            .zip(right)
            .into_iter()
            .map(|(left, right)| {
                (left.parse::<i32>())
                    .unwrap()
                    .abs_diff(right.parse::<i32>().unwrap()) as i32
            })
            .sum()
    }

    fn second(&self, input: &str) -> i32 {
        let input = input
            .lines()
            .filter_map(|line| {
                if let Some((left, right)) = line.split_once(char::is_whitespace) {
                    Some((left.trim().to_string(), right.trim().to_string()))
                } else {
                    None
                }
            })
            .collect::<(HashSet<_>, Vec<_>)>();
        let (left, right) = input;

        let right = right.into_iter().fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        });

        left.into_iter()
            .map(|value| match right.get(&value) {
                Some(mul) => value.parse::<i32>().unwrap() * *mul,
                None => 0,
            })
            .sum::<i32>()
    }
}
