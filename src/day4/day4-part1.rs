use advent_of_code_2019_rust::helpers::converters::to_isize;

fn has_pair(number: &Vec<isize>) -> bool {
    let mut ans = false;
    number.iter().enumerate().skip(1).for_each(|(idx, n)| {
        if *n == number[idx - 1] { ans = true }
    });
    ans
}

fn are_equal_or_incremental(number: &Vec<isize>) -> bool {
    let mut last = -1;
    let mut ans = true;
    number.iter().enumerate().for_each(|(idx, n)| {
        if last > *n { ans = false; }
        last = *n;
    });
    ans
}

fn number_to_vec(number: isize) -> Vec<isize> {
//    fn recur(number: isize, acc: &mut Vec<isize>) -> Vec<isize> {
//        if true {
//            let carry = number / 10;
//            let digit = number % 10;
//            acc.push(digit);
//            recur(carry, acc)
//        } else {
//            acc.to_vec()
//        }
//    }
//    let mut v = vec![];
//    recur(number, &v)
    let char_vec = number.to_string();
    char_vec.split("").filter(|f| !f.is_empty() ).map(|d| to_isize(d)).collect()
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

#[test]
fn sample_are_equal_or_incremental() {
    assert!(are_equal_or_incremental(&vec![1, 2, 3, 4 ,5, 6]), "incremental or equal");
    assert!(are_equal_or_incremental(&vec![1, 1, 2, 2, 3, 3]), "incremental or equal");
    assert!(!are_equal_or_incremental(&vec![3, 4, 5, 5 , 4, 3]), "not incremental or equal");
    assert!(!are_equal_or_incremental(&vec![6, 2, 5, 7 ,1, 6]), "not incremental or equal");
}

#[test]
fn sample_number_to_vecs() {
    println!("Hi");
    assert_eq!(number_to_vec(111111), vec![1, 1, 1, 1, 1, 1]);
    assert_eq!(number_to_vec(223450), vec![2, 2, 3, 4, 5, 0]);
    assert_eq!(number_to_vec(123789), vec![1, 2, 3, 7, 8, 9]);
    println!("{:?}", number_to_vec(111111));
}