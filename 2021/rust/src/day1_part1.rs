use advent_of_code_2021_rust::utils::file::load_integers_from_file;
use advent_of_code_2021_rust::day1_common::speed_of_depth_increase;

fn main() {
    let sea_floor_depths = load_integers_from_file("./data/day1.txt").expect("could not load file");
    println!("speed of depth increase: {}", speed_of_depth_increase(sea_floor_depths));
}

#[test]
fn example() {
    let sea_floor_depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, speed_of_depth_increase(sea_floor_depths));
}
