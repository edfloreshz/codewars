pub fn digital_root(n: i64) -> i64 {
    let n = n.to_string().chars().map(|num| num.to_digit(10).unwrap()).sum::<u32>() as i64;
    if n/10 >= 1 { return digital_root(n) }
    n
}