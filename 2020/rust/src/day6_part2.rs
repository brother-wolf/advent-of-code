extern crate regex;

use std::collections::HashMap;
use std::io;

use advent_of_code_2020_rust::day6_common::CustomsAnswers;
use advent_of_code_2020_rust::utils::file::lines_from_file;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day6.txt")?;
    println!("{:?}", CorrectCustomsAnswers.process(&lines));
    Ok(())
}

struct CorrectCustomsAnswers;
impl CustomsAnswers for CorrectCustomsAnswers {
    fn count(&self, s: &str, passengers: usize) -> usize {
        s.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).or_insert_with(Vec::new).push(c);
            acc
        }).iter().fold(0, |acc, (_c, v)| acc + if v.len() == passengers { 1 } else { 0 })
    }
}


#[test]
fn example1_test() -> io::Result<()> {
    let lines = lines_from_file("data/day6-example1.txt")?;
    assert_eq!(6, CorrectCustomsAnswers.process(&lines));
    Ok(())
}


#[test]
fn count_test() {

    assert_eq!(3, CorrectCustomsAnswers.count("abc", 1),"first");
    assert_eq!(1, CorrectCustomsAnswers.count("aaa", 3), "second");
    assert_eq!(2, CorrectCustomsAnswers.count("aabccde", 2), "third");
    assert_eq!(2, CorrectCustomsAnswers.count("aabcc", 2), "fourth");
    assert_eq!(1, CorrectCustomsAnswers.count("abccdefg", 2), "fifth");
}

#[test]
fn sandbox() {

}