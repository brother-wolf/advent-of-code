use itertools::Itertools;
use advent_of_code_2022_rust::utils::file::lines_from_file;
use advent_of_code_2022_rust::utils::testing_utils::{assert_contains_the_same, assert_contains_the_same_chars};

fn expand_assignment(inp: &str) -> Vec<usize> {
    let range = inp.split_once("-").unwrap();
    (range.0.parse::<usize>().unwrap()..=range.1.parse::<usize>().unwrap()).collect()
}

fn split_section_pairs(inp: &str) -> (Vec<usize>, Vec<usize>) {
    let (l, r) = inp.split_once(",").unwrap();
    (expand_assignment(l), expand_assignment(r))
}

fn is_contained(f:fn(usize, usize) -> bool, inp: (Vec<usize>, Vec<usize>)) -> bool {
    fn contains(f:fn(usize, usize) -> bool, longer: Vec<usize>, shorter: Vec<usize>) -> bool {
        let count = shorter.iter()
            .map(|rv| longer.contains(rv))
            .map(|b| if b { 1 } else { 0 })
            .sum::<usize>();
        f(count, shorter.len())
    }

    if inp.0.len() > inp.1.len() {
        contains(f, inp.0, inp.1)
    } else {
        contains(f, inp.1, inp.0)
    }
}

fn fully_overlapping(left: usize, right: usize) -> bool {
    left == right
}

fn partial_overlapping(left: usize, _: usize) -> bool {
    left > 0
}

fn count_assignment_pair_overlaps(f: fn(usize, usize) -> bool, data: &Vec<String>) -> usize {
    data.iter()
        .map(|s| if is_contained(f, split_section_pairs(s)) {1}else{0})
        .sum()
}

fn main() {
    let input_data_file = "data/day4.txt";
    match lines_from_file(input_data_file) {
        Ok(data) => {
            let fully_contained_count = count_assignment_pair_overlaps(fully_overlapping, &data);
            println!("count of assignment pair overlaps: {}", fully_contained_count);
            let overlapped_count = count_assignment_pair_overlaps(partial_overlapping, &data);
            println!("count of assignment pair overlaps: {}", overlapped_count);
        }
        Err(e) => println!("could not load data file {}, reason: {:?}", input_data_file, e),
    };
}

#[test]
fn should_find_assignment_pairs_that_fully_overlap() {
    let example_data = lines_from_file("data/day4-example.txt").unwrap();
    let count = count_assignment_pair_overlaps(fully_overlapping, &example_data);
    assert_eq!(2, count);
}

#[test]
fn should_find_assignment_pairs_that_partially_overlap() {
    let example_data = lines_from_file("data/day4-example.txt").unwrap();
    let count = count_assignment_pair_overlaps(partial_overlapping, &example_data);
    assert_eq!(4, count);
}