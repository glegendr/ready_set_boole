#[allow(dead_code)]
pub fn map(x: u16, y: u16) -> f64 {
    f64::from_bits(((x as u64) << 16) + (y as u64))
}


#[cfg(test)]
mod map {

    use std::collections::HashSet;

    use super::*;
    #[test]
    fn map_test() {
        let mut set = HashSet::new();
        let test_values = vec![
            (u16::MIN, u16::MIN),
            (u16::MAX, u16::MIN),
            (u16::MIN, u16::MAX),
            (u16::MAX, u16::MAX),
            (12324, 13),
            (421, 421),
            (42, 42),
            (421, 422),
            (422, 421),
        ];
        for (x, y) in test_values {
            if !set.insert(map(x, y).to_string()) {
                assert_eq!(1, 2);
            }
        }
        if set.iter().map(|s| s.parse::<f64>()).any(|f| match f {
            Ok(f) => f > 1. || f < 0.,
            _ => true
        }) {
                assert_eq!(2, 3);
        }
    }
}