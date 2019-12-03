pub fn to_i64(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or(0)
}