use advent_of_code_2019_rust::day4::common::{number_to_vec, are_equal_or_incremental};

pub fn has_pair(number: &Vec<isize>) -> bool {
    let mut ans = false;
    let mut idx: usize = 0;
    while idx < number.len() {
        match last_occurrence_of_same_number(number, idx) {
            Some(last) => {
                if last - idx == 1 {
                    ans = true;
                    idx = number.len();
                } else {
                    idx = last;
                }
            },
            None => idx += 1,
        }
    }
    ans
}

fn last_occurrence_of_same_number(number: &Vec<isize>, curr: usize) -> Option<usize> {
    let mut next_idx = curr + 1;
    while next_idx < number.len() && number[next_idx] == number[curr] {
//        println!("{}:{}", curr, next_idx)
        next_idx += 1;
    }
    next_idx-=1;
    if next_idx != curr { Some(next_idx)} else {None}
}

fn is_passcode(number: isize) -> bool {
    let num_vec = number_to_vec(number);
    are_equal_or_incremental(&num_vec) && has_pair(&num_vec)
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
    assert!(has_pair(&vec![1,1,2,2,3,3]), "should meet these criteria because the digits never decrease and all repeated digits are exactly two digits long");
    assert!(!has_pair(&vec![1,2,3,4,4,4]), "should no longer meets the criteria (the repeated 44 is part of a larger group of 444");
    assert!(has_pair(&vec![1,1,1,1,2,2]), "should meet the criteria (even though 1 is repeated more than twice, it still contains a double 22");
}

#[test]
fn sample_last_occurrence() {
    assert_eq!(last_occurrence_of_same_number(&vec![1, 1, 2, 3, 4, 5], 0).unwrap(), 1, "should be 1st index");
    assert_eq!(last_occurrence_of_same_number(&vec![1, 1, 2, 3, 4, 5], 1), None, "should not match");
    assert_eq!(last_occurrence_of_same_number(&vec![1, 1, 2, 2, 4, 5], 2).unwrap(), 3, "should be 3rd index");
    assert_eq!(last_occurrence_of_same_number(&vec![1, 1, 2, 2, 2, 5], 2).unwrap(), 4, "should be 4th index");
    assert_eq!(last_occurrence_of_same_number(&vec![1, 1, 2, 2, 5, 5], 5), None, "should be None");
    assert_eq!(last_occurrence_of_same_number(&vec![2, 2, 2, 2, 2, 2], 0).unwrap(), 5, "should be 5th index");
    assert_eq!(last_occurrence_of_same_number(&vec![2, 2, 2, 2, 2, 2], 1).unwrap(), 5, "should be 5th index");
    assert_eq!(last_occurrence_of_same_number(&vec![2, 2, 2, 2, 2, 3], 1).unwrap(), 4, "should be 4th index");
}
