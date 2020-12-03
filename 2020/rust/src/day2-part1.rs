use std::io;

use advent_of_code_2020_rust::day2_common::PasswordChecker;
use advent_of_code_2020_rust::utils::file::lines_from_file;

mod day2_common;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day2.txt")?;
    println!("{:?}", SledRentalPolicy {}.how_many_valid_passwords(&lines));
    Ok(())
}

struct SledRentalPolicy;

impl PasswordChecker for SledRentalPolicy {
    fn is_password_valid(&self, line: &str, c: char, min: usize, max: usize) -> bool {
        matches!(line.split("").into_iter().map(|slice| {
        let does_match = matches!(slice.chars().last(), Some(slice_char) if c == slice_char);
        if does_match { 1 } else { 0 }
    }).sum(), x if (min..max+1).contains(&x))
    }
}


#[test]
fn password_test() {
    assert!(SledRentalPolicy.is_password_valid("abcde", 'a', 1, 3 ));
    assert!(!SledRentalPolicy.is_password_valid("cdefg", 'b', 1, 3 ));
    assert!(SledRentalPolicy.is_password_valid("ccccccccc", 'c', 2, 9 ));

    assert!(SledRentalPolicy.is_password_valid("aaa", 'a', 1, 4 ));
    assert!(!SledRentalPolicy.is_password_valid("aaa", 'f', 1, 3 ));
    assert!(SledRentalPolicy.is_password_valid("aad", 'a', 0, 3 ));
}


