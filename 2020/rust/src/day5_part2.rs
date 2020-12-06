use std::io;

use advent_of_code_2020_rust::utils::file::lines_from_file;
use advent_of_code_2020_rust::day5_common::find_taken_seats;

fn main() -> io::Result<()> {
    let lines = lines_from_file("data/day5.txt")?;
    let mut taken_seats = find_taken_seats(&lines);
    taken_seats.sort();

    let max_idx = taken_seats.len() - 1;
    let possibles = taken_seats.iter().enumerate().filter(|(idx, seat)| {
        if *idx + 2 > max_idx { false }
        else {
            taken_seats.contains(&(*seat + 2)) &&
                !taken_seats.contains(&(*seat + 1))
        }
    })
        .map(|(idx, seat)| *seat + 1)
        .collect::<Vec<usize>>();
    println!("{:?}", possibles);
    Ok(())
}
