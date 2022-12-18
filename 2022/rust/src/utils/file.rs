use std::path::Path;
use std::io;
use std::str::FromStr;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


pub fn load_integers_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<isize>> {
    Ok(lines_from_file(filename)
        .expect("Could not load lines")
        .iter()
        .map(|s| isize::from_str(s).unwrap())
        .collect::<Vec<isize>>())
}