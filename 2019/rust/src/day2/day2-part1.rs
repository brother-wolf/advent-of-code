use advent_of_code_2019_rust::helpers::files::read_file;
use advent_of_code_2019_rust::helpers::converters::to_usize;
use advent_of_code_2019_rust::day2::compute::compute;

fn main() {
    let mut data: Vec<usize> = read_file("./data/day2/input.txt").split(",").map(|s| to_usize(s)).collect();
    data[1] = 12;
    data[2] = 2;
    println!("{}", compute(data)[0]);
}
