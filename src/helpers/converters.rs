pub fn to_i64(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or(0)
}
pub fn to_usize(s: &str) -> usize { s.parse::<usize>().unwrap_or(0) }
pub fn to_isize(s: &str) -> isize { s.parse::<isize>().unwrap_or(0) }