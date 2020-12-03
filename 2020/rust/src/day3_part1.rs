use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;
use advent_of_code_2020_rust::day3_common::toboggan;

fn main() -> io::Result<()> {
    println!("trees: {}", toboggan(1,3, &lines_from_file("data/day3.txt")?));
    Ok(())
}

#[test]
fn example_test() -> io::Result<()> {
    assert_eq!(7, toboggan(1,3,&lines_from_file("data/day3-example1.txt")?));
    Ok(())
}