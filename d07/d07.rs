use std::borrow::Cow;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Equation<'a> {
    result: u64,
    nums: Cow<'a, [u64]>,
}

impl Equation<'_> {
    fn from_str(s: &str) -> Option<Self> {
        let mut it = s.split(':');
        let result = it.next()?.trim().parse().ok()?;
        let nums: Vec<_> = it.next()?.split_whitespace().flat_map(str::parse).collect();
        Some(Equation {
            result,
            nums: nums.into(),
        })
    }
}

fn next_fac_10(i: u64) -> u64 {
    let mut f = 1;
    while i >= f {
        f *= 10;
    }
    f
}

impl Equation<'_> {
    fn is_possible(&self, part2: bool) -> bool {
        if self.nums.is_empty() {
            return false;
        }
        if let [n] = self.nums[..] {
            return n == self.result;
        }
        let last = *self.nums.last().unwrap();
        if self.result.is_multiple_of(last)
            && (Equation {
                result: self.result / last,
                nums: self.nums[..self.nums.len() - 1].into(),
            }
            .is_possible(part2))
        {
            return true;
        }
        if self.result >= last
            && (Equation {
                result: self.result - last,
                nums: self.nums[..self.nums.len() - 1].into(),
            }
            .is_possible(part2))
        {
            return true;
        }

        if part2 {
            let fac = next_fac_10(last);
            if self.result > last
                && (self.result - last).is_multiple_of(fac)
                && (Equation {
                    result: self.result / fac,
                    nums: self.nums[..self.nums.len() - 1].into(),
                }
                .is_possible(part2))
            {
                return true;
            }
        }

        false
    }
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let equations: Vec<_> = input.lines().flat_map(Equation::from_str).collect();
    let sum: u128 = equations
        .iter()
        .filter(|e| e.is_possible(false))
        .map(|e| e.result as u128)
        .sum();
    println!("Part1: {sum}");

    let sum: u128 = equations
        .iter()
        .filter(|e| e.is_possible(true))
        .map(|e| e.result as u128)
        .sum();
    println!("Part2: {sum}");
}

#[cfg(test)]
mod x {
    use super::*;

    macro_rules! equation {
        ($i:literal: $($nums:literal)+) => {
            Equation {
                result: $i,
                nums: [$($nums ,)+].to_vec().into(),
            }
        };
    }

    #[test]
    fn test_next_fac_10() {
        assert_eq!(next_fac_10(1), 10);
        assert_eq!(next_fac_10(9), 10);
        assert_eq!(next_fac_10(10), 100);
        assert_eq!(next_fac_10(99), 100);
        assert_eq!(next_fac_10(100), 1000);
    }

    #[test]
    fn test_part2() {
        let e = equation!(156: 15 6);
        assert!(!e.is_possible(false));
        assert!(e.is_possible(true));
    }
}
