use cargo_snippet::snippet;

#[snippet]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(18, 12), 6);
        assert_eq!(gcd(12, 0), 12);
        assert_eq!(gcd(0, 12), 12);
        assert_eq!(gcd(0, 0), 0);
    }
}
