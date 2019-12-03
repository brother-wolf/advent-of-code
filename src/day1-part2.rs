use advent_of_code_2019_rust::helpers::files::read_file;
use advent_of_code_2019_rust::helpers::converters::to_i64;

fn fuel_required(mass: i64) -> i64 {
    fn recur(mass: i64, acc: i64) -> i64 {
        let a = ((mass as f64 / 3.0) as i64) - 2;
        if a <= 0 { acc } else { recur(a, acc + a) }
    }
    recur(mass, 0)
}

fn main() {
    let contents = read_file("./data/day1/input.txt");

    let result: i64 = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter().filter(|s| !s.is_empty())
        .map(|s| fuel_required(to_i64(s)))
        .sum::<i64>();

    println!("{:?}", result);
}

// A module of mass 14 requires 2 fuel. This fuel requires no further fuel
// (2 divided by 3 and rounded down is 0, which would call for a negative fuel),
// so the total fuel required is still just 2.
#[test]
fn sample_small_module() {
    assert_eq!(fuel_required(14), 2);
}

// At first, a module of mass 1969 requires 654 fuel. Then, this fuel
// requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel,
// which requires 21 fuel, which requires 5 fuel, which requires no further fuel.
// So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
#[test]
fn sample_medium_module() {
    assert_eq!(fuel_required(1969), 966);
}

// The fuel required by a module of mass 100756 and its fuel is:
// 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
#[test]
fn sample_large_module() {
    assert_eq!(fuel_required(100756), 50346);
}