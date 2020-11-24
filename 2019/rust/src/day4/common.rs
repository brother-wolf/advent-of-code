use crate::helpers::converters::to_isize;

pub fn are_equal_or_incremental(number: &Vec<isize>) -> bool {
    let mut last = -1;
    let mut ans = true;
    number.iter().for_each(|n| {
        if last > *n { ans = false; }
        last = *n;
    });
    ans
}

pub fn number_to_vec(number: isize) -> Vec<isize> {
    let char_vec = number.to_string();
    char_vec.split("").filter(|f| !f.is_empty() ).map(|d| to_isize(d)).collect()
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
