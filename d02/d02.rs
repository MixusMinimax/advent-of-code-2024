use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let rows: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    fn pair_is_valid(a: i32, b: i32, ordering: &mut Option<Ordering>) -> bool {
        if !(1..=3).contains(&(a - b).unsigned_abs()) {
            return false;
        }
        let ord = a.cmp(&b);
        if ord == Ordering::Equal {
            return false;
        }
        if let Some(o) = *ordering
            && o != ord
        {
            return false;
        }
        *ordering = Some(ord);
        true
    }

    let c = rows
        .iter()
        .filter(|r| {
            let mut ordering = None;
            r.iter()
                .copied()
                .tuple_windows()
                .all(|(a, b)| pair_is_valid(a, b, &mut ordering))
        })
        .count();
    println!("Part1: {c}");

    let c = rows
        .iter()
        .filter(|r| {
            (0..r.len()).any(|i| {
                let mut ordering = None;
                r.iter()
                    .take(i)
                    .chain(r.iter().skip(i + 1))
                    .copied()
                    .tuple_windows()
                    .all(|(a, b)| pair_is_valid(a, b, &mut ordering))
            })
        })
        .count();
    println!("Part2: {c}");
}
