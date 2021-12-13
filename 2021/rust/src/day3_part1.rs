use advent_of_code_2021_rust::utils::file::lines_from_file;

fn bin_to_dec(binary: &Vec<usize>) -> usize {
    binary.iter().rev().enumerate().map(|(idx, &val)|
        val * 2_usize.pow(idx as u32)
    ).sum()
}

fn analyse_diagnostics(diagnostics_report: Vec<&str>) -> (usize, usize, usize) {
    let min_len = diagnostics_report.iter().last().unwrap().len();
    let init_vec = (0..min_len).map(|_| 0).collect::<Vec<usize>>();
    let report_len = diagnostics_report.len() as isize;
    let vec = diagnostics_report
        .iter()
        .map(|s| s.chars().map(|c| (c == '1') as usize))
        .fold(init_vec, |acc, row| row.zip(acc).map(|(a, b)| a + b).collect())
        .iter()
        .map(|&u| ((u as isize) > (report_len - u as isize)) as usize).collect();
    let gamma = bin_to_dec(&vec);
    let epsilon = bin_to_dec(&vec.iter().map(|u| 1 - u).collect());
    (gamma,epsilon,gamma*epsilon)
}

fn main() {
    let diagnostics_report = lines_from_file("./data/day3.txt").expect("could not load file");
    let (gamma, epsilon, power_consumption) = analyse_diagnostics(diagnostics_report.iter().map(|s| s.as_str()).collect());
    println!("gamma: {}  epsilon: {}  power consumption: {}", gamma, epsilon, power_consumption);
}

#[test]
fn example() {
    let diagnostic_report = vec![
        "00100", "11110", "10110", "10111",
        "10101", "01111", "00111", "11100",
        "10000", "11001", "00010", "01010",
    ];

    let (gamma, epsilon, power_consumption) = analyse_diagnostics(diagnostic_report);

    assert_eq!(22, gamma);
    assert_eq!(9, epsilon);
    assert_eq!(198, power_consumption)
}


#[test]
fn binary_vec_should_convert_to_decimal_correctly() {
    assert_eq!(22, bin_to_dec(&vec![1,0,1,1,0]));
    assert_eq!(9, bin_to_dec(&vec![0,1,0,0,1]));
    assert_eq!(30, bin_to_dec(&vec![1,1,1,1,0]));
}

