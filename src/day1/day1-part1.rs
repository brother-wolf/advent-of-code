use advent_of_code_2019_rust::helpers::files::read_file;
use advent_of_code_2019_rust::helpers::converters::to_i64;

fn fuel_required(mass: i64) -> i64 {
    ((mass as f64 / 3.0) as i64) - 2
}

fn main() {
    let contents = read_file("./data/day1/input.txt");

    let result: i64 = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter().filter(|s| !s.is_empty())
        .map(|s| fuel_required(to_i64(s)))
        .sum::<i64>();

    println!("{:?}", result);
}

// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
#[test]
fn sample_small_mass() {
    assert_eq!(fuel_required(12), 2);
}

// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
#[test]
fn another_sample_small_mass() {
    assert_eq!(fuel_required(14), 2);
}

// For a mass of 1969, the fuel required is 654.
#[test]
fn sample_medium_mass() {
    assert_eq!(fuel_required(1969), 654);
}

// For a mass of 100756, the fuel required is 33583.
#[test]
fn sample_large_mass() {
    assert_eq!(fuel_required(100756), 33583);
}

