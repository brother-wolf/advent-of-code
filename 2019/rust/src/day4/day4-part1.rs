use advent_of_code_2019_rust::day4::common::{number_to_vec, are_equal_or_incremental};

pub fn has_pair(number: &Vec<isize>) -> bool {
    let mut ans = false;
    number.iter().enumerate().skip(1).for_each(|(idx, n)| {
        if *n == number[idx - 1] { ans = true }
    });
    ans
}

fn is_passcode(number: isize) -> bool {
    let num_vec = number_to_vec(number);
    has_pair(&num_vec) &&
        are_equal_or_incremental(&num_vec)
}

fn main() {
    let answers: Vec<isize> = (264360..746325).flat_map(|number| {
        if is_passcode(number) { Some(number) } else { None }
    }).collect();
    println!("{:?}", answers);
    println!("total: {}", answers.len());
}


#[test]
fn sample_passcodes() {
    assert!(is_passcode(111111), "Should be valid: double 11, never decreases");
    assert!(!is_passcode(223450), "Should be invalid: decreasing pair of digits 50");
    assert!(!is_passcode(123789), "Should be invalid: no double");
}

#[test]
fn sample_has_pair() {
    assert!(!has_pair(&vec![1, 2, 3, 4 ,5, 6]), "has no pair");
    assert!(has_pair(&vec![1, 1, 2, 2, 3, 3]), "has pair");
    assert!(has_pair(&vec![3, 4, 5, 5 , 4, 3]), "has pair");
    assert!(!has_pair(&vec![6, 2, 5, 7 ,1, 6]), "has no pair");
}