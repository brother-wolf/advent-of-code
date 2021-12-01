pub fn speed_of_depth_increase(sea_floor_depths: Vec<isize>) -> isize {
    if sea_floor_depths.len() == 0 { return 0; }
    let (head, tail) = sea_floor_depths.split_at(1);
    tail.iter().fold((0, head.first().unwrap()), |(acc, last_depth), curr_depth| {
        if curr_depth > last_depth { (acc + 1, curr_depth) } else { (acc, curr_depth) }
    }).0
}

pub fn windowed_depths(sea_floor_depths: Vec<isize>, window_size: usize) -> Vec<isize> {
    if sea_floor_depths.len() < window_size { return vec![]; }
    (window_size..= sea_floor_depths.len()).fold(vec![], |mut acc, idx|{
        acc.push(sea_floor_depths.split_at(idx - window_size).1.split_at(window_size).0.iter().sum());
        acc
    })
}

#[test]
fn zero_readings_should_return_zero_depth() {
    assert_eq!(0, speed_of_depth_increase(vec![]));
}

#[test]
fn one_reading_should_return_zero_depth() {
    assert_eq!(0, speed_of_depth_increase(vec![199]));
}

#[test]
fn two_identical_readings_should_return_zero_depth_count() {
    assert_eq!(0, speed_of_depth_increase(vec![199, 199]));
}

#[test]
fn decreasing_readings_should_return_zero_depth_count() {
    assert_eq!(0, speed_of_depth_increase(vec![199, 198]));
}

#[test]
fn increasing_readings_should_return_zero_depth_count() {
    assert_eq!(1, speed_of_depth_increase(vec![197, 198]));
}

#[test]
fn example_windowed_depths() {
    let sea_floor_depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let expected_depths = vec![607,618,618,617,647,716,769,792];
    let actual = windowed_depths(sea_floor_depths, 3);
    assert_eq!(&expected_depths, &actual);
}

#[test]
fn zero_readings_should_return_empty_collection() {
    let sea_floor_depths = vec![];
    let expected: Vec<isize> = vec![];
    assert_eq!(expected, windowed_depths(sea_floor_depths, 3));
}

#[test]
fn one_reading_should_return_empty_collection() {
    let sea_floor_depths = vec![199];
    let expected: Vec<isize> = vec![];
    assert_eq!(expected, windowed_depths(sea_floor_depths, 3));
}

#[test]
fn two_readings_should_return_empty_collection() {
    let sea_floor_depths = vec![199, 200];
    let expected: Vec<isize> = vec![];
    assert_eq!(expected, windowed_depths(sea_floor_depths, 3));
}

#[test]
fn three_readings_should_return_zero_depth() {
    let sea_floor_depths = vec![199, 200, 201];
    assert_eq!(vec![600], windowed_depths(sea_floor_depths, 3));
}

#[test]
fn three_readings_with_window_of_two_should_return_depth_count() {
    let sea_floor_depths = vec![199, 200, 201];
    assert_eq!(vec![399, 401], windowed_depths(sea_floor_depths, 2));
}

#[test]
fn four_readings_with_window_of_two_should_return_three_depths() {
    let sea_floor_depths = vec![199, 200, 199, 300];
    assert_eq!(vec![399, 399, 499], windowed_depths(sea_floor_depths, 2));
}
