extern crate regex;

use std::io;
use std::str::FromStr;

use regex::Regex;

use advent_of_code_2020_rust::utils::file::lines_from_file;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day2.txt")?;
    println!("{:?}", how_many_valid_passwords(&lines));
    Ok(())
}

fn how_many_valid_passwords(lines: &Vec<String>) -> usize {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    lines.iter().map(|line| {
        let cap = re.captures_iter(line).last().unwrap();
        if is_password_valid(
            &cap[4],
            cap[3].chars().nth(0).unwrap(),
            usize::from_str(&cap[1]).unwrap(),
            usize::from_str(&cap[2]).unwrap() + 1
            ) { 1 } else { 0 }
    }).sum()
}

fn is_password_valid(line: &str, c: char, min: usize, max: usize) -> bool {
    matches!(line.split("").into_iter().map(|slice| {
        let does_match = matches!(slice.chars().last(), Some(slice_char) if c == slice_char);
        if does_match { 1 } else { 0 }
    }).sum(), x if (min..max).contains(&x))
}

#[test]
fn password_test() {
    assert!(is_password_valid("aaa", 'a', 1, 4 ));
    assert!(!is_password_valid("aaa", 'f', 1, 3 ));
    assert!(is_password_valid("aad", 'a', 0, 3 ));
}


