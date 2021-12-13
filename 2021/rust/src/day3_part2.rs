use advent_of_code_2021_rust::utils::file::lines_from_file;

fn is_one_common(value: usize, len: isize) -> bool {
    (len - value as isize) < value as isize
}

fn analyse_o2_and_co2(diagnostics_report: Vec<&str>) -> (usize, usize, usize) {
    fn filterer(report: Vec<&str>, idx: usize, matcher: char) -> Vec<&str> {
        report.into_iter().filter(|a| a.chars().nth(idx).unwrap() == matcher).collect::<Vec<&str>>()
    }

    fn recurse(report: Vec<&str>, idx: usize, least_common: bool) -> Option<String> {
        match report.len() {
            0 => None,
            1 => match report.get(0) {
                Some(&s) => Some(s.to_string()),
                None => None,
            },
            _ => {
                let numerical_vec = report.iter()
                    .map(|a| (a.chars().nth(idx).unwrap() == '0') as usize)
                    .collect::<Vec<usize>>();
                let common = is_one_common(numerical_vec.iter().sum(), numerical_vec.len() as isize);
                let matcher = if common == least_common { '1' } else { '0' };
                recurse(filterer(report, idx, matcher), idx + 1, least_common)
            }
        }
    }

    let oxygen = usize::from_str_radix(
        recurse(diagnostics_report.clone(), 0, false)
            .unwrap().as_str(), 2).unwrap();
    let co2 = usize::from_str_radix(
        recurse(diagnostics_report, 0, true)
            .unwrap().as_str(), 2).unwrap();
    (oxygen, co2, oxygen * co2)
}

fn main() {
    let diagnostic_report = lines_from_file("./data/day3.txt")
        .expect("could not load file");
    let (oxygen, carbon_dioxide, life_support_rating) =
        analyse_o2_and_co2(
            diagnostic_report.iter()
                .map(|a| a.as_str())
                .collect());
    println!("oxygen             : {}", oxygen);
    println!("carbon dioxide     : {}", carbon_dioxide);
    println!("life support rating: {}", life_support_rating);
}


#[test]
fn example() {
    let diagnostic_report = vec![
        "00100", "11110", "10110", "10111",
        "10101", "01111", "00111", "11100",
        "10000", "11001", "00010", "01010",
    ];

    let (oxygen, carbon_dioxide, life_support_rating) =
        analyse_o2_and_co2(diagnostic_report);

    assert_eq!((23, 10), (oxygen, carbon_dioxide));
    assert_eq!(230, life_support_rating)
}
