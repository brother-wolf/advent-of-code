pub fn compute(mut data: Vec<usize>) -> Vec<usize> {
    let mut p = 0;
    while p < data.len() {
        if p + 3 > data.len() { break; }

        let update_index = data[p + 3];
        match data[p] {
            1 => data[update_index] = data[data[p + 1]] + data[data[p + 2]],
            2 => data[update_index] = data[data[p + 1]] * data[data[p + 2]],
            _ => break,
        }
        p += 4;
    }
    data
}

#[test]
fn process_two_cycle_process_correctly() {
    let data: Vec<usize> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let expected: Vec<usize> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(compute(data), expected);
}

#[test]
fn process_small_program_one() {
    let data: Vec<usize> = vec![1, 0, 0, 0, 99];
    let expected: Vec<usize> = vec![2, 0, 0, 0, 99];
    assert_eq!(compute(data), expected);
}

#[test]
fn process_small_program_two() {
    let data: Vec<usize> = vec![2, 3, 0, 3, 99];
    let expected: Vec<usize> = vec![2, 3, 0, 6, 99];
    assert_eq!(compute(data), expected);
}

#[test]
fn process_small_program_three() {
    let data: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
    let expected: Vec<usize> = vec![2, 4, 4, 5, 99, 9801];
    assert_eq!(compute(data), expected);
}

#[test]
fn process_two_cycle_process_number_two() {
    let data: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let expected: Vec<usize> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
    assert_eq!(compute(data), expected);
}
