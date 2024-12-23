use std::collections::{HashMap, HashSet};

use crate::solvable::Solvable;

pub struct Day05;

impl Solvable for Day05 {
    fn first(&self, input: &str) -> i64 {
        let (rules, updates) = self.parse_input(input);
        let mut sum = 0;

        for update in updates.iter() {
            if self.is_valid_update(&rules, update) {
                let middle_page = update[update.len() / 2];
                sum += middle_page;
            }
        }

        sum
    }

    fn second(&self, input: &str) -> i64 {
        let (rules, updates) = self.parse_input(input);

        let invalid_updates = updates
            .into_iter()
            .filter(|update| !self.is_valid_update(&rules, update))
            .collect::<Vec<_>>();

        let mut sum = 0;

        for update in invalid_updates {
            let mut depends_on: Graph = HashMap::new();
            let mut dependents: Graph = HashMap::new();
            let mut no_deps = HashSet::new();

            for &page in &update {
                depends_on.entry(page).or_insert_with(HashSet::new);
                dependents.entry(page).or_insert_with(HashSet::new);
                no_deps.insert(page);
            }

            for &(x, y) in &rules {
                if update.contains(&x) && update.contains(&y) {
                    add_edge(&mut depends_on, y, x);
                    add_edge(&mut dependents, x, y);
                    no_deps.remove(&y);
                }
            }

            let mut state = State {
                depends_on,
                dependents,
                no_deps: no_deps.into_iter().collect::<Vec<_>>(),
            };

            let sorted_update = self.topological_sort(&mut state);

            let middle_page = sorted_update[sorted_update.len() / 2];
            sum += middle_page;
        }

        sum
    }
}

impl Day05 {
    fn parse_input(&self, input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
        let split = input.split("\n\n").collect::<Vec<_>>();
        let rules = split[0];
        let updates = split[1];

        let rules = rules
            .lines()
            .map(|line| {
                let rule = line
                    .split('|')
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();

                (rule[0], rule[1])
            })
            .collect::<Vec<_>>();

        let updates = updates
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|u| u.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        (rules, updates)
    }

    fn is_valid_update(&self, rules: &[(i64, i64)], update: &[i64]) -> bool {
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

    fn topological_sort(&self, state: &mut State) -> Vec<i64> {
        let mut res = vec![];

        while let Some(node) = state.no_deps.pop() {
            res.push(node);

            if let Some(dependents) = state.get_dependents(&node) {
                for dependent in dependents.clone() {
                    state.resolve(&dependent, &node);
                }
            }
        }

        res
    }
}

type Graph = HashMap<i64, HashSet<i64>>;

struct State {
    depends_on: Graph,
    dependents: Graph,
    no_deps: Vec<i64>,
}

impl State {
    fn resolve(&mut self, dependent: &i64, dependency: &i64) {
        if let Some(deps) = self.depends_on.get_mut(dependent) {
            deps.remove(dependency);

            if deps.is_empty() {
                self.no_deps.push(*dependent);
                self.depends_on.remove(dependent);
            }
        }
    }

    fn get_dependents(&self, dependency: &i64) -> Option<&HashSet<i64>> {
        self.dependents.get(dependency)
    }
}

fn add_edge(graph: &mut Graph, from: i64, to: i64) {
    graph
        .entry(from)
        .and_modify(|pointees| {
            pointees.insert(to);
        })
        .or_insert_with(|| {
            let mut s = HashSet::new();
            s.insert(to);
            s
        });
}
