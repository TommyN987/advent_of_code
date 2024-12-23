use crate::solutions::{Day01, Day02, Day03, Day04, Day05, Day06, Day07};

pub trait Solvable {
    fn first(&self, input: &str) -> i64;
    fn second(&self, input: &str) -> i64;
}

pub struct Registry {
    solvers: Vec<Box<dyn Solvable>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut solvers: Vec<Box<dyn Solvable>> = Vec::with_capacity(25);
        solvers.push(Box::new(Day01));
        solvers.push(Box::new(Day02));
        solvers.push(Box::new(Day03));
        solvers.push(Box::new(Day04));
        solvers.push(Box::new(Day05));
        solvers.push(Box::new(Day06));
        solvers.push(Box::new(Day07));

        Self { solvers }
    }

    pub fn solve(&self, inputs: &[String]) -> Vec<(i64, i64)> {
        self.solvers
            .iter()
            .zip(inputs)
            .map(|(solver, input)| (solver.first(input), solver.second(input)))
            .collect()
    }
}
