extern crate regex;

use std::str::FromStr;

use regex::Regex;

pub trait PasswordChecker {
    fn is_password_valid(&self, line: &str, c: char, min: usize, max: usize) -> bool;

    fn how_many_valid_passwords(&self, lines: &Vec<String>) -> usize {
        let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        lines.iter().map(|line| {
            let cap = re.captures_iter(line).last().unwrap();
            if self.is_password_valid(
                &cap[4],
                cap[3].chars().nth(0).unwrap(),
                usize::from_str(&cap[1]).unwrap(),
                usize::from_str(&cap[2]).unwrap()
            ) { 1 } else { 0 }
        }).sum()
    }
}