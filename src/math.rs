use num::{Signed, pow};
use vecmath::Vector2;

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

pub fn vec2_hamming<S: Copy + Signed>([ax, ay]: Vector2<S>, [bx, by]: Vector2<S>) -> S {
    ax.abs_sub(&bx) + ay.abs_sub(&by)
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

    #[test]
    fn test_vec2_hamming() {
        assert_eq!(vec2_hamming::<isize>([5, 0], [2, -1]), 4);
    }
}
