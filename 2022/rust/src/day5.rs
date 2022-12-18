use itertools::Itertools;
use advent_of_code_2022_rust::utils::file::lines_from_file;
use advent_of_code_2022_rust::utils::testing_utils::{assert_contains_the_same, assert_contains_the_same_chars};

fn move_stack(num: usize, from: usize, to: usize, data: &mut Vec<Vec<char>>) {
    let mut moved_crates = vec![];
    (0..num).for_each(|_| {
        let _crate = (&mut data.get_mut(from -1).unwrap()).pop().unwrap();
        moved_crates.push(_crate);
    });
    moved_crates.reverse();
    moved_crates.iter().for_each(|c| data.get_mut(to -1).unwrap().push(*c));
}

fn main() {
    let input_data_file = "data/day5.txt";
    match lines_from_file(input_data_file) {
        Ok(data) => {
            let (commands, mut state) = process_input(&data);
            let mut final_state = process_commands_crate_mover_9000(&commands, &state);
            let top = top_crates(&mut final_state);
            println!("top crates are: {}", top);
            let mut final_state = process_commands_crate_mover_9001(&commands, &state);
            let top = top_crates(&mut final_state);
            println!("top crates are: {}", top);
        }
        Err(e) => println!("could not load data file {}, reason: {:?}", input_data_file, e),
    };
}

fn process_commands_crate_mover_9000(commands: &Vec<(usize, usize, usize)>, starting_point: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut data = starting_point.clone();
    commands.iter().for_each(|(num, from, to)| (0..*num).for_each(|_| move_stack(1, *from, *to, &mut data)));
    data
}

fn process_commands_crate_mover_9001(commands: &Vec<(usize, usize, usize)>, starting_point: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut data = starting_point.clone();
    commands.iter().for_each(|(num, from, to)| move_stack(*num, *from, *to, &mut data));
    data
}

fn top_crates(data: &mut Vec<Vec<char>>) -> String {
    data.iter().flat_map(|v| v.last()).join("")
}

fn process_input(data: &Vec<String>) -> (Vec<(usize, usize, usize)>, Vec<Vec<char>>) {
    use regex::Regex;
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let last_digit_re = Regex::new( r"(\d+) *$").unwrap();

    let mut commands: Vec<(usize, usize, usize)> = vec![];
    let mut start_point: Vec<Vec<char>> = vec![];
    let mut num_stacks = 0;
    let mut mutable_data = data.clone();
    mutable_data.reverse();
    mutable_data.iter().for_each(|row|{
        println!("{:?}", row);
        match row {
            r if move_re.is_match(r) => {
                let cap = move_re.captures(r).unwrap();
                println!("cap[0]: {:?}, cap[1]: {:?}, cap[2]: {:?}, cap[3]: {:?}", &cap[0], &cap[1], &cap[2], &cap[3]);
                commands.push((*&cap[1].parse::<usize>().unwrap(), *&cap[2].parse::<usize>().unwrap(), *&cap[3].parse::<usize>().unwrap()))
            },
            r if r.starts_with(" 1") => {
                let cap = last_digit_re.captures(r).unwrap();
                num_stacks = *&cap[&cap.len() -1].parse::<usize>().unwrap();
                (0..num_stacks).for_each(|_| start_point.push(vec![]));
            },
            r if r.contains("[") => {
                (0..num_stacks).for_each(|stack| {
                    match r.chars().nth((stack * 4) + 1) {
                        Some(crt) if crt != ' ' => start_point.get_mut(stack).unwrap().push(crt),
                        _ => (),
                    };
                })
            },
            _ => (),
        }
    });
    commands.reverse();
    (commands, start_point)
}


#[test]
fn should_process_the_example_input_file() {
    let expected_commands = vec![(1usize, 2usize, 1usize), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
    let expected_start_point: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

    let mut example_data = lines_from_file("data/day5-example.txt").unwrap();
    let (commands, start_point) = process_input(&mut example_data);
    println!("commands            : {:?}", &commands);
    println!("expected commands   : {:?}", &expected_commands);
    println!("start_point         : {:?}", &start_point);
    println!("expected_start_point: {:?}", &expected_start_point);
}

#[test]
fn should_check_moving_crates_one_by_one() {
    let mut example_data = lines_from_file("data/day5-example.txt").unwrap();
    let (commands, mut state) = process_input(&mut example_data);
    let mut final_state = process_commands_crate_mover_9000(&commands, &mut state);
    let ans = top_crates(&mut final_state);
    assert_eq!("CMZ", ans.as_str());
}

#[test]
fn should_check_moving_crates_as_group() {
    let mut example_data = lines_from_file("data/day5-example.txt").unwrap();
    let (commands, mut state) = process_input(&mut example_data);
    let mut final_state = process_commands_crate_mover_9001(&commands, &mut state);
    let ans = top_crates(&mut final_state);
    assert_eq!("MCD", ans.as_str());
}
