use itertools::Itertools;
use advent_of_code_2022_rust::utils::file::lines_from_file;
use advent_of_code_2022_rust::utils::testing_utils::assert_contains_the_same;

fn accumulate_calories_carried_by_fold(data: &Vec<String>) -> Vec<usize> {
    let mut results = data.iter().fold((vec![], 0usize), |(mut out, acc), value| {
        match value.parse::<usize>() {
            Ok(next) => (out, acc + next),
            Err(_) => {
                out.push(acc);
                (out, 0)
            }
        }
    });
    results.0.push(results.1);
    results.0
}

fn largest(data: &Vec<usize>) -> (usize, usize) {
    let (idx, cals) = data.iter().enumerate().fold((0, 0), |(idx, count), (next_idx, next_count)| {
        if count >= *next_count { (idx, count) } else { (next_idx, *next_count) }
    });
    (idx + 1, cals)
}

fn total_top_n(data: &Vec<usize>, n: usize) -> usize {
    let mut local_data = data.clone();
    local_data.sort();
    local_data.reverse();
    local_data.as_slice()[0..n].iter().sum()
}

fn main() {
    let input_data_file = "data/day1.txt";
    match lines_from_file(input_data_file) {
        Ok(mut data) => {
            let collected_calories = accumulate_calories_carried_by_fold(&mut data);
            let (idx, calories) = largest(&collected_calories);
            println!("elf number {} has the highest calorific bounty at {:?}", idx, calories);
            let n = 3;
            let top_n = total_top_n(&collected_calories, n);
            println!("top {} elves have a calorific count at {:?}", n, top_n);
        },
        Err(e) => println!("could not load data file {}, reason: {:?}", input_data_file, e),
    };
}

#[test]
fn example_data_should_sum_the_calories_by_elf_correctly() {
    let example_data = lines_from_file("data/day1-example.txt").unwrap();
    let expected = vec![6000, 4000, 11000, 24000, 10000];
    let actual = accumulate_calories_carried_by_fold(&example_data);
    assert_contains_the_same(&expected, &actual);
}

#[test]
fn example_data_should_return_highest_calorie_carrying_elf() {
    let example_data = lines_from_file("data/day1-example.txt").unwrap();
    let accumulated_calories = accumulate_calories_carried_by_fold(&example_data);

    let (idx, calories) = largest(&accumulated_calories);

    assert_eq!(idx, 4, "elf number is not correct");
    assert_eq!(calories, 24000, "calorie count is not correct");
}

#[test]
fn example_data_should_return_n_number_highest_calorie_of_carrying_elf() {
    let example_data = lines_from_file("data/day1-example.txt").unwrap();
    let accumulated_calories = accumulate_calories_carried_by_fold(&example_data);

    let top_3_calorie_count = total_top_n(&accumulated_calories, 3);

    assert_eq!(top_3_calorie_count, 45000, "sum of calories for highest 3 calorie carrying elves is not correct");
}