#[cfg(test)]
#[test]
fn test_to_i64() {
    use super::to_i64;
    assert_eq!(to_i64("1234".as_bytes()), 1234);
    assert_eq!(to_i64("01244".as_bytes()), 1244);
}
