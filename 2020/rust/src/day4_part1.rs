extern crate regex;

use std::io;

use regex::Regex;

use advent_of_code_2020_rust::utils::file::lines_from_file;

fn main() -> io::Result<()> {
    let input_file = "data/day4.txt";
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let lines = lines_from_file(input_file)?;

    let count = number_valid_passports(&lines, &valid);
    println!("{:?}", count);
    Ok(())
}

fn validate_doc(doc: &Vec<&str>, expected_values: &Vec<&str>) -> usize {
    expected_values.iter().map(|expected| doc.contains(expected) as usize).product::<usize>()
}

fn extract_fields(row: &str) -> Vec<&str> {
    let re = Regex::new(r"(\w+):[^ ]+").unwrap();
    re.captures_iter(row).into_iter().map(|cap| cap.get(1).map_or("", |m| m.as_str())).collect::<Vec<&str>>()
}

fn number_valid_passports(lines: &Vec<String>, expected_values: &Vec<&str>) -> usize {
    let mut count: usize = 0;
    let mut doc = vec![];
    let mut lines_iter = lines.iter();
    loop {
        match lines_iter.next() {
            Some(line) => {
                if line.is_empty() {
                    count += validate_doc(&doc, expected_values);
                    doc = vec![];
                } else {
                    extract_fields(line).iter().for_each(|item| doc.push(*item));
                }
            }
            None => {
                count += validate_doc(&doc, expected_values);
                break;
            }
        }
    }
    count
}

#[test]
fn example_test() -> io::Result<()> {
    let lines = lines_from_file("data/day4-example1.txt")?;
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    assert_eq!(2, number_valid_passports(&lines, &valid));
    Ok(())
}

#[test]
fn extract_fields_test() {
    let row = "iyr:2010 ecl:gry hgt:181cm pid:591597745 byr:1920 hcl:#6b5442 eyr:2029 cid:123";
    let actuals = extract_fields(row);

    vec!["iyr", "ecl", "hgt", "pid", "byr", "hcl", "eyr", "cid"]
        .iter()
        .zip(actuals)
        .for_each(|(expected, actual)| {
            assert_eq!(*expected, actual)
        });
}

#[test]
fn valid_doc_test() {
    let valid = vec!["aaa", "bbb", "ccc"];
    assert_eq!(0, validate_doc(&vec!["aaa", "bbb"], &valid));
    assert_eq!(0, validate_doc(&vec!["aaa"], &valid));
    assert_eq!(1, validate_doc(&vec!["aaa", "bbb", "ccc"], &valid));
    assert_eq!(1, validate_doc(&vec!["aaa", "ddd", "bbb", "ccc"], &valid));
}