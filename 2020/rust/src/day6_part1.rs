use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day6.txt")?;
    println!("{:?}", process(&lines));
    Ok(())
}

fn count(s: &str) -> usize {
    let mut c = s.chars().collect::<Vec<char>>();
    c.sort();
    c.dedup();
    c.len()
}

fn process(lines: &Vec<String>) -> usize {
    let mut num: usize = 0;
    let mut buf = String::new();

    let mut lines_iter = lines.iter();
    loop {
        match lines_iter.next() {
            Some(line) => {
                if line.is_empty() {
                    num += count(&buf);
                    buf = String::new();
                } else {
                    buf.push_str(line);
                }
            },
            None => {
                num += count(&buf);
                break;
            },
        }
    }
    num
}

#[test]
fn example1_test() -> io::Result<()> {
    let lines = lines_from_file("data/day6-example1.txt")?;
    assert_eq!(11, process(&lines));
    Ok(())
}

#[test]
fn count_test() {
    assert_eq!(3, count("abc"));
    assert_eq!(1, count("aaa"));
    assert_eq!(5, count("aabccde"));
    assert_eq!(3, count("aabcc"));
    assert_eq!(7, count("abccdefg"));
}