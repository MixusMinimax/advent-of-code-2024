use aoc2016::math::{next_fac_10, pow10};
use std::collections::HashMap;

fn blink_once(input: impl IntoIterator<Item = u64>) -> impl Iterator<Item = u64> {
    input.into_iter().flat_map(|i| match i {
        0 => [1, 0].into_iter().take(1),
        _ => {
            let (digits, _) = next_fac_10(i);
            if digits.is_multiple_of(2) {
                let fac = pow10(digits / 2);
                [i / fac, i % fac].into_iter().take(2)
            } else {
                [i * 2024, 0].into_iter().take(1)
            }
        }
    })
}

/// This is in `O(2*n)`, which is terrible.
#[allow(dead_code)]
fn blink_n_times(input: Vec<u64>, n: usize) -> Vec<u64> {
    let mut v1 = input;
    let mut v2 = Vec::with_capacity(v1.len());
    for i in 0..n {
        let (src, dst) = if i.is_multiple_of(2) {
            (&v1, &mut v2)
        } else {
            (&v2, &mut v1)
        };
        dst.clear();
        dst.extend(blink_once(src.iter().copied()));
    }
    if n.is_multiple_of(2) { v1 } else { v2 }
}

/// We recurse down the `2*n` tree, dfs. If we ever reach a number with a value we already
/// encountered at that depth, we do not descend into it, we simply reuse the already calculated
/// value. This way, the width of each layer is no longer in `O(2*n)`, but very limited.
/// Any duplicate value we find skips an entire `O(2*n)` subtree.
/// This turns something that won't finish in my lifetime into something that finishes
/// in 100ms in a dev build.
///
/// Caching in recursive algorithms is incredibly powerful.
///
/// On recursion: recursion is not the best choice, but it is fine to use when the recursion
/// depth is very limited, which is hard-limited to `n`, which in our example is 75.
fn blink_count_cached(input: &[u64], n: usize) -> usize {
    let mut cache = HashMap::new();

    fn recurse(cache: &mut HashMap<(u64, usize), usize>, i: u64, n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        if let Some(&c) = cache.get(&(i, n)) {
            return c;
        }
        let c = match i {
            0 => recurse(cache, 1, n - 1),
            _ => {
                let (digits, _) = next_fac_10(i);
                if digits.is_multiple_of(2) {
                    let fac = pow10(digits / 2);
                    recurse(cache, i / fac, n - 1) + recurse(cache, i % fac, n - 1)
                } else {
                    recurse(cache, i * 2024, n - 1)
                }
            }
        };
        cache.insert((i, n), c);
        c
    }

    let mut total = 0;

    for &i in input {
        total += recurse(&mut cache, i, n);
    }

    total
}

fn main() {
    let input = include_str!("input.txt");
    let numbers: Vec<_> = input
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    let count = blink_count_cached(&numbers, 25);
    println!("Part1: {}", count);
    let count = blink_count_cached(&numbers, 75);
    println!("Part2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink_once() {
        let v: Vec<_> = blink_once([0, 1, 10, 99, 999]).collect();
        assert_eq!(v, [1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_blink_1() {
        let mut v = vec![125, 17];
        v = blink_once(v).collect();
        assert_eq!(v, [253000, 1, 7]);
        v = blink_once(v).collect();
        assert_eq!(v, [253, 0, 2024, 14168]);
        v = blink_once(v).collect();
        assert_eq!(v, [512072, 1, 20, 24, 28676032]);
    }

    #[test]
    fn test_blink_cached() {
        let v = vec![125, 17];
        assert_eq!(blink_count_cached(&v, 1), 3);
        assert_eq!(blink_count_cached(&v, 2), 4);
        assert_eq!(blink_count_cached(&v, 3), 5);
    }
}
