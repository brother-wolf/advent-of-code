use advent_of_code_2021_rust::utils::file::load_integers_from_file;
use advent_of_code_2021_rust::day1_common::{speed_of_depth_increase, windowed_depths};

fn main() {
    let sea_floor_depths = load_integers_from_file("./data/day1.txt").expect("could not load file");
    println!("windowed speed of depth increase: {:?}", speed_of_depth_increase(windowed_depths(sea_floor_depths, 3)));
}

#[test]
fn example() {
    let sea_floor_depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(5, speed_of_depth_increase(windowed_depths(sea_floor_depths, 3)));
}
