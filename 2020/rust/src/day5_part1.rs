use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day5.txt")?;
    let highest_seat_number = lines.iter().map(|line| find_seat_id(line)).max().unwrap();
    println!("{:?}", highest_seat_number);
    Ok(())
}

fn find_seat_id(code: &String) -> usize {
    let (row_code, col_code) = code.split_at(7);
    (find_row(row_code) * 8) + find_col(col_code)
}

fn process(code: &str, max: usize, lower: char, upper: char) -> usize {
    let mut min = 0;
    let mut mx = max;
    for c in code.chars() {
        if c == upper {min = min + ((mx - min) / 2)}
        else if c == lower { mx = mx - ((mx - min) / 2) }
        else {  panic!(format!("Should not have got anything other than an F or B, got '{}'", c))}
    };
    mx - 1
}

fn find_row(code: &str) -> usize {
    process(code, 128, 'F', 'B')
}

fn find_col(code: &str) -> usize {
    process(code, 8, 'L', 'R')
}

#[test]
fn find_seat_id_test()  {
    assert_eq!(567, find_seat_id(&String::from("BFFFBBFRRR")));
    assert_eq!(119, find_seat_id(&String::from("FFFBBBFRRR")));
    assert_eq!(820, find_seat_id(&String::from("BBFFBBFRLL")));
}

#[test]
fn find_row_test() {
    assert_eq!(70, find_row("BFFFBBF"));
    assert_eq!(14, find_row("FFFBBBF"));
    assert_eq!(102, find_row("BBFFBBF"));
}

#[test]
fn find_col_test() {
    assert_eq!(7, find_col("RRR"));
    assert_eq!(7, find_col("RRR"));
    assert_eq!(4, find_col("RLL"));
}

#[test]
fn single_test() {
    assert_eq!(44, find_row("FBFBBFF"));
    assert_eq!(5, find_col("RLR"));
    assert_eq!(357, find_seat_id(&String::from("FBFBBFFRLR")));
}
