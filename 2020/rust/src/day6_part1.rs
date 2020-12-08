use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;
use advent_of_code_2020_rust::day6_common::CustomsAnswers;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day6.txt")?;
    println!("{:?}", MisreadCustomsAnswers.process(&lines));
    Ok(())
}

struct MisreadCustomsAnswers;
impl CustomsAnswers for MisreadCustomsAnswers {
    fn count(&self, s: &str, _passengers: usize) -> usize {
        let mut c = s.chars().collect::<Vec<char>>();
        c.sort();
        c.dedup();
        c.len()
    }
}


#[test]
fn example1_test() -> io::Result<()> {
    let lines = lines_from_file("data/day6-example1.txt")?;
    assert_eq!(11, MisreadCustomsAnswers.process(&lines));
    Ok(())
}

#[test]
fn count_test() {

    assert_eq!(3, MisreadCustomsAnswers.count("abc", 0));
    assert_eq!(1, MisreadCustomsAnswers.count("aaa", 0));
    assert_eq!(5, MisreadCustomsAnswers.count("aabccde", 0));
    assert_eq!(3, MisreadCustomsAnswers.count("aabcc", 0));
    assert_eq!(7, MisreadCustomsAnswers.count("abccdefg", 0));
}