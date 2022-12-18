pub fn assert_contains_the_same(expected: &Vec<usize>, actual: &Vec<usize>) {
    assert_eq!(expected.len(), actual.len(), "do not contain the same number of elements");
    actual.iter().enumerate().for_each(|(idx, value)| assert_eq!(expected[idx], *value, "elements do not match"));
}

pub fn assert_contains_the_same_chars(expected: &Vec<char>, actual: &Vec<char>) {
    assert_eq!(expected.len(), actual.len(), "do not contain the same number of elements");
    actual.iter().enumerate().for_each(|(idx, value)| assert_eq!(expected[idx], *value, "elements do not match"));
}