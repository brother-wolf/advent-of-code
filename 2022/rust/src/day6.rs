use itertools::Itertools;
use advent_of_code_2022_rust::utils::file::lines_from_file;
use advent_of_code_2022_rust::utils::testing_utils::{assert_contains_the_same, assert_contains_the_same_chars};


fn find_start_marker(data: &str, window_size: usize) -> usize {
    data.chars()
        .collect_vec()
        .as_mut_slice()
        .windows(window_size)
        .map(|a|
            if a.iter()
                .map(|left| {
                    if 1 == a.iter()
                        .fold(0, |acc, right| if left == right { acc + 1 } else { acc })
                    { 1 } else { 0 }
                }).sum::<usize>() == window_size { 1 } else { 0 }
        ).enumerate().filter(|(idx, count)| *count == 1).map(|(idx, count)| idx).nth(0).unwrap() + window_size
}

fn main() {
    let input_data_file = "data/day6.txt";
    match lines_from_file(input_data_file) {
        Ok(data) => {
            match data.first() {
                Some(data) => {
                    let start_of_packet_marker = find_start_marker(&data, 4);
                    println!("start of packet marker is: {}", start_of_packet_marker);
                    let start_of_message_marker = find_start_marker(&data, 14);
                    println!("start of message marker is: {}", start_of_message_marker);
                }
                None => println!("loaded data file {} contains no data rows", input_data_file),
            }
        }
        Err(e) => println!("could not load data file {}, reason: {:?}", input_data_file, e),
    };
}


#[test]
fn should_find_start_of_packet_marker() {
    let data = vec!(
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11));

    data.iter().enumerate()
        .for_each(|(idx, (input_data, expected_start_marker))| {
            println!("test #{}: {:?}", idx, input_data);
            assert_eq!(*expected_start_marker, find_start_marker(input_data, 4));
        });
}

#[test]
fn should_find_start_of_message_marker() {
    let data = vec!(
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)
    );
    data.iter().enumerate()
        .for_each(|(idx, (input_data, expected_start_marker))| {
            println!("test #{}: {:?}", idx, input_data);
            assert_eq!(*expected_start_marker, find_start_marker(input_data, 14));
        });
}