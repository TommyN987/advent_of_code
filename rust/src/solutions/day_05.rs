use std::collections::HashMap;

use crate::solvable::Solvable;

pub struct Day05;

impl Solvable for Day05 {
    fn first(&self, input: &str) -> i32 {
        let split = input.split("\n\n").collect::<Vec<_>>();
        let rules = split[0];
        let updates = split[1];

        let rules = rules
            .lines()
            .map(|line| {
                let rule = line
                    .split('|')
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                (rule[0], rule[1])
            })
            .collect::<Vec<_>>();

        let updates = updates
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|u| u.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut sum = 0;

        for update in updates.iter() {
            if self.is_valid_update(&rules, update) {
                let middle_page = update[update.len() / 2];
                sum += middle_page;
            }
        }

        sum
    }

    fn second(&self, input: &str) -> i32 {
        0
    }
}

impl Day05 {
    fn is_valid_update(&self, rules: &[(i32, i32)], update: &[i32]) -> bool {
        let index_map = update
            .iter()
            .enumerate()
            .map(|(i, &page)| (page, i))
            .collect::<HashMap<_, _>>();

        for &(x, y) in rules {
            if let (Some(&x_index), Some(&y_index)) = (index_map.get(&x), index_map.get(&y)) {
                if x_index >= y_index {
                    return false;
                }
            }
        }

        true
    }
}
