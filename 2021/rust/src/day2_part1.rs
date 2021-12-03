use advent_of_code_2021_rust::day2_common::parse_data;
use advent_of_code_2021_rust::utils::file::lines_from_file;

fn calc_position_and_depth(data: &Vec<(isize, isize)>) -> (isize, isize) {
    data.iter().fold((0, 0), | (y, x), (ny, nx)| (y + ny, x + nx))
}

fn main() {
    let directions = lines_from_file("./data/day2.txt").expect("could not load file");
    let data= parse_data(&directions.iter().map(|a| a.as_str()).collect());
    let (y, x) = calc_position_and_depth(&data);
    println!("ans: {} x {} = {}",y, x, y * x);
}


#[test]
fn example() {
    let directions = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];
    let data= parse_data(&directions);
    let (y, x) = calc_position_and_depth(&data);
    assert_eq!((10, 15, 150), (y, x, y*x));
}
