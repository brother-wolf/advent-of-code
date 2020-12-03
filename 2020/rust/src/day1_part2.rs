use std::io;

use advent_of_code_2020_rust::utils::file::load_integers_from_file;

fn main() -> io::Result<()> {
    let mut lines = load_integers_from_file("data/day1.txt")?;
    lines.sort();
    let ints = lines.clone();
    let mut ans = (0,0,0);
    for l in &ints {
        match get_pair_for_total(2020 - l, &ints) {
            Ok(pair) => {
                ans = (*l, pair.0, pair.1);
                break;
            },
            Err(_) => (),
        }
    }

    println!("{} + {} + {} = {}", ans.0, ans.1, ans.2, ans.0 + ans.1 + ans.2);
    println!("{} * {} * {} = {}", ans.0, ans.1, ans.2, ans.0 * ans.1 * ans.2);

    Ok(())
}

fn get_pair_for_total(target: isize, lines: &Vec<isize>) -> Result<(isize, isize), String> {
    for l in lines.iter() {
        let search_term = target - l;
        match lines.binary_search(&search_term) {
            Ok(pair) => {
                return Ok((*l, lines[pair]))
            },
            Err(_e) => (),
        }
    }
    Err(String::from("No Result"))
}


#[test]
fn get_pair_for_total_should_return_the_same_as_part_1() {
    let mut lines = load_integers_from_file("data/day1.txt").unwrap();
    lines.sort();
    let actual = get_pair_for_total(2020, &mut lines).unwrap();

    assert_eq!(actual.0 * actual.1, 121396);
}