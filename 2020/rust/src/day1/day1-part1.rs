use std::io;

use advent_of_code_2020_rust::utils::file::load_integers_from_file;

fn main() -> io::Result<()> {
    let mut lines = load_integers_from_file("data/day1/part1.txt")?;
    lines.sort();

    let mut ans = (0,0);
    for l in lines.iter() {
        let search_term = 2020 - l;
        match lines.binary_search(&search_term) {
            Ok(pair) => {
                ans = (*l, lines[pair]);
                break;
            },
            Err(_e) => (),
        }
    }

    println!("{:?}", ans.0 * ans.1 );
    Ok(())
}