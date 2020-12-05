extern crate regex;

use std::io;

use regex::Regex;

use advent_of_code_2020_rust::utils::file::lines_from_file;
use advent_of_code_2020_rust::day4_common::ValidDocCounter;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let input_file = "data/day4.txt";
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let lines = lines_from_file(input_file)?;

    let count = DetailedValidDocCounter.number_valid_passports(&lines, &valid);
    println!("{:?}", count);
    Ok(())
}

pub struct DetailedValidDocCounter;
impl DetailedValidDocCounter {
    fn field_validator(&self, field: &str, value: &str) -> bool {
        match field {
            "byr" => DetailedValidDocCounter::is_between(value, 1920, 2002),//`1920` and at most `2002`
            "iyr" => DetailedValidDocCounter::is_between(value, 2010, 2020),//`2010` and at most `2020`
            "eyr" => DetailedValidDocCounter::is_between(value, 2020, 2030),//  `2020` and at most `2030`
            "hgt" => {
                let cm_in_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
                match cm_in_re.captures_iter(value).next() {
                    Some(cap) => match (cap.get(2), cap.get(1)) {
                        (Some(field), Some(value)) => match field.as_str() {
                            "cm" => DetailedValidDocCounter::is_between(value.as_str(), 150, 193),
                            "in" => DetailedValidDocCounter::is_between(value.as_str(), 59, 76),
                            _ => false,
                        },
                        _ => false,
                    },
                    None => false,
                }
            },
            "hcl" => {
                let colour_re = Regex::new(r"^#([a-f0-9]+)$").unwrap();
                colour_re.captures_iter(value).next().is_some()
            },
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            "pid" => value.len() == 9 && usize::from_str(&value).is_ok(),
            _ => false,
        }
    }

    fn is_between(value: &str, min: usize, max: usize) -> bool {
        match usize::from_str(value) {
            Ok(num) => num >= min && num <= max,
            Err(_) => false,
        }
    }
}
impl ValidDocCounter for DetailedValidDocCounter {
    fn validate_doc(&self, doc: &Vec<(&str, &str)>, _: &Vec<&str>) -> usize {
        (doc.iter().map(|(field, value)| {
            self.field_validator(field, value) as usize
        }).sum::<usize>() == 7) as usize
    }
}


#[test]
fn invalid_example_test() -> io::Result<()> {
    let lines = lines_from_file("data/day4-example2-invalid.txt")?;
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    assert_eq!(0, DetailedValidDocCounter.number_valid_passports(&lines, &valid));
    Ok(())
}

#[test]
fn valid_example_test() -> io::Result<()> {
    let lines = lines_from_file("data/day4-example2-valid.txt")?;
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    assert_eq!(4, DetailedValidDocCounter.number_valid_passports(&lines, &valid));
    Ok(())
}

#[test]
fn run_test() -> io::Result<()> {
    let lines = lines_from_file("data/day4.txt")?;
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    assert_eq!(184, DetailedValidDocCounter.number_valid_passports(&lines, &valid));
    Ok(())
}