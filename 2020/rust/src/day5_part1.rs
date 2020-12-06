use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;
use advent_of_code_2020_rust::day5_common::find_taken_seats;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day5.txt")?;
    let taken_seats = find_taken_seats(&lines);
    let highest_seat_number = taken_seats.iter().max().unwrap();
    println!("{:?}", highest_seat_number);
    Ok(())
}
