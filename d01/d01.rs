use itertools::Itertools;
use std::ops::Sub;

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let (mut l, mut r): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| -> (i32, i32) {
            let mut it = l.split_whitespace();
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    l.sort_unstable();
    r.sort_unstable();
    let total: u64 = l
        .iter()
        .zip(&r)
        .map(|(&a, &b)| a.sub(b).unsigned_abs() as u64)
        .sum();
    println!("Part1: {total}");

    let l = l.into_iter().counts();
    let r = r.into_iter().counts();
    let total: u64 = l
        .keys()
        .chain(r.keys())
        .copied()
        .unique()
        .map(|k| {
            k as u64
                * l.get(&k).copied().unwrap_or_default() as u64
                * r.get(&k).copied().unwrap_or_default() as u64
        })
        .sum();
    println!("Part2: {total}");
}
