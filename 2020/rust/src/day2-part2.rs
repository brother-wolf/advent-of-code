use std::io;

use advent_of_code_2020_rust::day2_common::PasswordChecker;
use advent_of_code_2020_rust::utils::file::lines_from_file;

mod day2_common;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day2.txt")?;
    println!("{:?}", TobogganCorporatePolicy {}.how_many_valid_passwords(&lines));
    Ok(())
}

struct TobogganCorporatePolicy;

impl PasswordChecker for TobogganCorporatePolicy {
    fn is_password_valid(&self, line: &str, c: char, min: usize, max: usize) -> bool {
        (line.chars().nth(min - 1).unwrap() == c) ^ (line.chars().nth(max - 1).unwrap() == c)
    }
}


#[test]
fn password_test() {
    assert!(TobogganCorporatePolicy{}.is_password_valid("abcde", 'a', 1, 3 ));
    assert!( ! TobogganCorporatePolicy{}.is_password_valid("cdefg", 'b', 1, 3 ));
    assert!( ! TobogganCorporatePolicy{}.is_password_valid("ccccccccc", 'c', 2, 9 ));
}


