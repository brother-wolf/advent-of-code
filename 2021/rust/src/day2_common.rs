use itertools::Itertools;

pub fn parse_data(directions: &Vec<&str>) -> Vec<(isize, isize)> {
    directions.iter()
        .flat_map(|s| match s.split(" ").collect_vec() {
            a if a.len() == 2 => {
                let x = a.first().unwrap();
                match a.last().unwrap().parse::<isize>() {
                    Ok(i) => match *x {
                        "forward" => Some((0, i)),
                        "down" => Some((i, 0)),
                        "up" => Some((-1 * i, 0)),
                        _ => None
                    },
                    Err(_) => None,
                }
            }
            _ => None,
        }).collect()
}

#[test]
fn should_map_directions_to_coords() {
    let directions = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let expected = vec![
         (0, 5),
         (5, 0),
         (0, 8),
         (-3, 0),
         (8, 0),
         (0, 2),
    ];

    assert_eq!(expected, parse_data(&directions));
}
