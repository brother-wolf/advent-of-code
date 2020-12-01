use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use std::str::FromStr;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn main() {
    let mut lines = lines_from_file("data/day1/part1.txt")
        .expect("Could not load lines")
        .iter()
        .map(|s| isize::from_str(s).unwrap())
        .collect::<Vec<isize>>();
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
}