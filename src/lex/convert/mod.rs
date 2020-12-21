mod tests;

pub fn to_i64(source: &[u8]) -> i64 {
    let mut res = 0;
    let mut is_zero = true;
    for element in source {
        let x = element.clone() as char;

        if x == '0' && is_zero {
            continue;
        }
        is_zero = false;

        res *= 10;
        res += x.to_digit(10).unwrap() as i64;
    }
    res
}
