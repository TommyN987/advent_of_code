use std::{
    fs::{read_dir, read_to_string},
    io,
};

use solvable::Registry;

mod day_01;
mod day_02;
mod solvable;

fn main() -> io::Result<()> {
    let source_dir = "./inputs/";
    let mut inputs = Vec::with_capacity(25);

    for entry in read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let input = read_to_string(path)?;
            inputs.push(input);
        }
    }

    let registry = Registry::new();

    let results = registry.solve(&inputs);

    results.iter().enumerate().for_each(|(i, (first, second))| {
        println!("Day {} ===> First: {}; Second: {}", i + 1, first, second);
    });

    Ok(())
}
