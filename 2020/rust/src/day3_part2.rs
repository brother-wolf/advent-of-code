use std::io;

use advent_of_code_2020_rust::day3_common::toboggan;
use advent_of_code_2020_rust::utils::file::lines_from_file;

const steps: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

fn main() -> io::Result<()> {
    let tree_lines = &lines_from_file("data/day3.txt")?;
    let trees_hit = steps.iter().map(|(down, right)| {
        toboggan(*down, *right, tree_lines)
    }).product::<usize>();
    println!("trees: {}", trees_hit);
    Ok(())
}

#[test]
fn example_test() -> io::Result<()> {
    let tree_lines = &lines_from_file("data/day3-example1.txt")?;

    assert_eq!(336, steps.iter().map(|(down, right)| {
        toboggan(*down, *right, tree_lines)
    }).product::<usize>());
    Ok(())
}