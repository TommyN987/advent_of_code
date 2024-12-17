use std::{
    fs::{read_dir, read_to_string},
    io,
    path::PathBuf,
};

use solvable::Registry;

mod solutions;
mod solvable;

fn main() -> io::Result<()> {
    let source_dir = "../inputs/";
    let mut inputs = Vec::with_capacity(25);

    let mut paths: Vec<PathBuf> = read_dir(source_dir)?
        .filter_map(|entry| entry.ok()) // Filter out any erroneous entries
        .filter(|entry| entry.path().is_file()) // Only keep files
        .map(|entry| entry.path()) // Extract the path
        .collect();

    // Sort the paths lexicographically (alphabetically)
    paths.sort();

    // Read and store the file contents
    for path in paths {
        let input = read_to_string(&path)?;
        inputs.push(input);
    }

    let registry = Registry::new();

    let results = registry.solve(&inputs);

    results.iter().enumerate().for_each(|(i, (first, second))| {
        println!("Day {} ===> First: {}; Second: {}", i + 1, first, second);
    });

    Ok(())
}
