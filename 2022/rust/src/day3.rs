use itertools::Itertools;
use advent_of_code_2022_rust::utils::file::lines_from_file;
use advent_of_code_2022_rust::utils::testing_utils::{assert_contains_the_same, assert_contains_the_same_chars};

fn split_into_compartments(data: &Vec<String>) -> Vec<(&str, &str)> {
    data.iter().map(|row| row.split_at(row.len() / 2)).collect()
}

fn find_common(a: &str, b: &str) -> Vec<char> {
    a.chars().filter(|c| b.contains(*c)).collect()
}

fn find_common_between_compartments(data: &Vec<(&str, &str)>) -> Vec<char> {
    data.iter().map(|(l, r)| *find_common(l, r).first().unwrap()).collect()
}

fn calculate_priority(data:&Vec<char>) -> usize {
    data.iter().map(|c|{
        c.to_uppercase().nth(0).unwrap() as usize - 64 + (if c.is_ascii_uppercase() { 26 } else {0} )
    }).sum()
}

fn find_group_badges(data: &Vec<String>) -> Vec<char> {
    fn find(data: &[String]) -> char {
        let first_common = find_common(data.get(0).unwrap(), data.get(1).unwrap()).into_iter().collect::<String>();
        *find_common(first_common.as_str(), data.get(2).unwrap()).first().unwrap()
    }
    data.chunks(3).map(|a| find(a)).collect()
}

fn main() {
    let input_data_file = "data/day3.txt";
    match lines_from_file(input_data_file) {
        Ok(mut data) => {
            let priority_sum = calculate_priority(&find_common_between_compartments(&split_into_compartments(&data)));
            println!("priority sum: {:?}", priority_sum);
            let badge_priority_sum = calculate_priority(&find_group_badges(&data));
            println!("priority sum: {:?}", badge_priority_sum);
        }
        Err(e) => println!("could not load data file {}, reason: {:?}", input_data_file, e),
    };
}

#[test]
fn should_split_the_compartments() {
    let example_data = lines_from_file("data/day3-example.txt").unwrap();
    split_into_compartments(&example_data).iter().for_each(|(l, r)| {
        assert_eq!(l.len(), r.len());
        assert_ne!(l, r);
    })
}

#[test]
fn should_find_the_common_items() {
    let example_data = lines_from_file("data/day3-example.txt").unwrap();
    let common_items = find_common_between_compartments(&split_into_compartments(&example_data));
    let expected = vec!['p', 'L', 'P', 'v', 't', 's'];
    assert_eq!(&expected.iter().join(""), &common_items.iter().join(""));
    println!("len of common items: '{:?}'", common_items.len());
    println!("common items: '{:?}'", common_items);
}

#[test]
fn should_calculate_priority() {
    let example_data = lines_from_file("data/day3-example.txt").unwrap();
    let priority_sum = calculate_priority(&find_common_between_compartments(&split_into_compartments(&example_data)));
    assert_eq!(157, priority_sum);
}

#[test]
fn should_find_the_group_badges() {
    let example_data = lines_from_file("data/day3-example.txt").unwrap();
    let group_badges = find_group_badges(&example_data);
    let expected = vec!['r', 'Z'];
    assert_contains_the_same_chars(&expected, &group_badges);
}


