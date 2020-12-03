use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;

fn toboggan(tree_lines: &Vec<String>) -> usize {
    let mut x = -3_isize;

    tree_lines.iter().map(|tree_line| {
        x+=3;
        if x >= (tree_line.len() as isize) { x -= tree_line.len() as isize }
        if x < 0 { panic!(format!("x is : {}", x )) }
        if tree_line.chars().nth(x as usize).unwrap() == '#' { 1 } else { 0 }
    }).sum::<usize>()
}

fn main() -> io::Result<()> {
    println!("trees: {}", toboggan(&lines_from_file("data/day3.txt")?));
    Ok(())
}

#[test]
fn example_test() -> io::Result<()> {
    assert_eq!(7, toboggan(&lines_from_file("data/day3-example1.txt")?));
    Ok(())
}