use std::collections::HashSet;

/// parse lines like `4|67` until an empty line is found.
///
/// returns an iterator pointing at the first line *after* the empty line.
fn parse_rules<'i>(
    mut lines: impl Iterator<Item = &'i str>,
) -> (impl Iterator<Item = &'i str>, HashSet<(u32, u32)>) {
    let res = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut it = l.split('|');
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    (lines, res)
}

/// I was thinking about creating a transitive ordering over the ruleset. But then I thought,
/// sorting the numbers themselves being in O(n^2) (this is literally bubblesort) is totally fine,
/// considering none of the number lists is longer than like 20 elements or so.
fn sort_numbers(mut numbers: Vec<u32>, rules: &HashSet<(u32, u32)>) -> Vec<u32> {
    let l = numbers.len();
    for a in 0..l - 1 {
        for b in a + 1..l {
            if rules.contains(&(numbers[b], numbers[a])) {
                numbers.swap(a, b);
            }
        }
    }
    numbers
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let (lines, rules) = parse_rules(input.lines());
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let numbers: Vec<_> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let sorted = sort_numbers(numbers.clone(), &rules);
        if numbers == sorted {
            part1 += numbers[numbers.len() / 2] as u64;
        } else {
            part2 += sorted[sorted.len() / 2] as u64;
        }
    }
    println!("Part1: {part1}");
    println!("Part2: {part2}");
}
