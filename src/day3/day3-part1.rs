extern crate itertools;

use advent_of_code_2019_rust::day3::geometry::Point;
use advent_of_code_2019_rust::helpers::files::read_file;
use advent_of_code_2019_rust::day3::generators::{build_path_from, find_intersection_points, manhattan_distance, ManhattanDistance};

fn main() {
    let data = read_file("./data/day3/input.txt");
    let lines = data.split("\n").collect::<Vec<&str>>();

    let wire_1 = lines[0].split(",").collect::<Vec<&str>>();
    let wire_2 = lines[1].split(",").collect::<Vec<&str>>();

    let closest_point = process(wire_1, wire_2);

    match closest_point {
        Some(closest_point) => println!("Manhattan distance: {}", closest_point.x.abs() + closest_point.y.abs()),
        None => println!("No intersection"),
    }
}

fn process(input_1: Vec<&str>, input_2: Vec<&str>) -> Option<Point> {

    let wire_1 = build_path_from(input_1);
    let wire_2 = build_path_from(input_2);

    let mut last: Vec<ManhattanDistance> = find_intersection_points(&wire_1, &wire_2).iter()
        .map(|inter| manhattan_distance(*inter)).collect::<Vec<ManhattanDistance>>();

    last.sort_by(|a, b| b.distance.cmp(&a.distance));

    match last.last() {
        Some(manhattan_distance) => Some(manhattan_distance.point),
        None => None
    }
}


#[test]
fn example_one_process() {
    let expected = Point {x:3, y:3};

    let wire_1 = vec!["R8","U5","L5","D3"];
    let wire_2 = vec!["U7","R6","D4","L4"];
    let actual = process(wire_1, wire_2);
    assert!(actual.is_some());
    assert_eq!(actual.unwrap(), expected);
}

