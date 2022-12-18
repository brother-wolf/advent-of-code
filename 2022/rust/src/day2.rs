use itertools::Itertools;
use advent_of_code_2022_rust::utils::file::lines_from_file;
use advent_of_code_2022_rust::utils::testing_utils::assert_contains_the_same;

fn split_data(data: &Vec<String>) -> Vec<(usize, usize)> {
    fn convert(c: char) -> usize { (c as usize - 64) % (87 - 64) }

    data.iter()
        .map(|r| (convert(r.chars().nth(0).unwrap()), convert(r.chars().nth(2).unwrap())))
        .collect()
}

fn given_the_shape_score(input: &Vec<(usize, usize)>) -> usize {
    input.iter().map(|(left, right)|
        match (*left, *right) {
            (l, r) if l == r => 3 + *right,
            (1, 2) | (2, 3) | (3, 1) => 6 + *right,
            _ => *right,
        }
    ).sum()
}

fn select_the_shape_score(input: &Vec<(usize, usize)>) -> usize {
    input.iter().map(|(left, right)|
        match *right {
            1 => (*left % 3 + 1) % 3 + 1, // choose the shape one lower than them in order to loose - account for wrap around!
            2 => *left, // choose same shape
            _ => *left % 3 + 1, // choose shape one higher than them in order to win - account for wrap around!
        } + ((*right - 1) * 3) // add 0 for loss, 3 for draw, 6 for win
    ).sum()
}

fn main() {
    let input_data_file = "data/day2.txt";
    match lines_from_file(input_data_file) {
        Ok(data) => {
            let split_data = split_data(&data);
            println!("my total score following strategy: {}", given_the_shape_score(&split_data));
            println!("my total score following for the win strategy: {}", select_the_shape_score(&split_data));
        }
        Err(e) => println!("could not load data file {}, reason: {:?}", input_data_file, e),
    };
}

#[test]
fn example_data_following_encrypted_strategy_guide_given_the_shape_playing_strategy() {
    let example_data = lines_from_file("data/day2-example.txt").unwrap();
    let expected_total = 15;
    let split_data = split_data(&example_data);
    assert_eq!(3, split_data.len());

    let actual_score = given_the_shape_score(&split_data);

    assert_eq!(expected_total, actual_score);
}

#[test]
fn example_data_following_encrypted_strategy_guide_for_selecting_the_shape_playing_strategy() {
    let example_data = lines_from_file("data/day2-example.txt").unwrap();
    let expected_total = 12;
    let split_data = split_data(&example_data);
    assert_eq!(3, split_data.len());

    let actual_score = select_the_shape_score(&split_data);

    assert_eq!(expected_total, actual_score);
}
