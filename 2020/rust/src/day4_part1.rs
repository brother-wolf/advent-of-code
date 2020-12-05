use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;
use advent_of_code_2020_rust::day4_common::ValidDocCounter;

fn main() -> io::Result<()> {
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let lines = lines_from_file("data/day4.txt")?;

    let count = SimpleValidDocCounter.number_valid_passports(&lines, &valid);
    println!("{:?}", count);
    Ok(())
}

struct SimpleValidDocCounter;

impl <'a> ValidDocCounter for SimpleValidDocCounter {
    fn validate_doc(&self, doc: &Vec<(&str, &str)>, expected_values: &Vec<&str>) -> usize {
        let fields = doc.iter().map(|(f, _)| *f).collect::<Vec<&str>>();
        expected_values.iter().map(|expected| fields.contains(expected) as usize).product::<usize>()
    }
}


#[test]
fn example_test() -> io::Result<()> {
    let lines = lines_from_file("data/day4-example1.txt")?;
    let valid = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    assert_eq!(2, SimpleValidDocCounter.number_valid_passports(&lines, &valid));
    Ok(())
}

#[test]
fn extract_fields_test() {
    let row = "iyr:2010 ecl:gry hgt:181cm pid:591597745 byr:1920 hcl:#6b5442 eyr:2029 cid:123";
    let actuals = SimpleValidDocCounter.extract_fields(row);

    vec!["iyr", "ecl", "hgt", "pid", "byr", "hcl", "eyr", "cid"]
        .iter()
        .zip(actuals)
        .for_each(|(expected, actual)| {
            assert_eq!(*expected, actual.0)
        });
}

#[test]
fn valid_doc_test() {
    let valid = vec!["aaa", "bbb", "ccc"];
    assert_eq!(0, SimpleValidDocCounter.validate_doc(&vec![("aaa", ""), ("bbb", "")], &valid));
    assert_eq!(0, SimpleValidDocCounter.validate_doc(&vec![("aaa", "")], &valid));
    assert_eq!(1, SimpleValidDocCounter.validate_doc(&vec![("aaa", ""), ("bbb", ""), ("ccc", "")], &valid));
    assert_eq!(1, SimpleValidDocCounter.validate_doc(&vec![("aaa", ""), ("ddd", ""), ("bbb", ""), ("ccc", "")], &valid));
}