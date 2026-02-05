use num::pow;

pub fn next_fac_10(n: u64) -> (u8, u64) {
    let mut e = 0;
    let mut f = 1;
    loop {
        if f > n {
            return (e, f);
        }
        e += 1;
        f *= 10;
    }
}

#[inline(always)]
pub fn pow10(e: impl Into<usize>) -> u64 {
    pow(10, e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_fac_10() {
        assert_eq!(next_fac_10(0), (0, 1));
        assert_eq!(next_fac_10(1), (1, 10));
        assert_eq!(next_fac_10(9), (1, 10));
        assert_eq!(next_fac_10(10), (2, 100));
        assert_eq!(next_fac_10(11), (2, 100));
        assert_eq!(next_fac_10(99), (2, 100));
        assert_eq!(next_fac_10(100), (3, 1000));
    }
}
