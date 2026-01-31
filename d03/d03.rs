use regex::Regex;

fn main() {
    // let input = include_str!("sample1.txt");
    // let input = include_str!("sample2.txt");
    let input = include_str!("input.txt");
    let pat = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
    let s: u64 = pat
        .captures_iter(input)
        .map(|c| c[1].parse::<u64>().unwrap() * c[2].parse::<u64>().unwrap())
        .sum();
    println!("Part1: {s}");

    let pat = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)"#).unwrap();
    let s: u64 = pat
        .captures_iter(input)
        .scan(true, |acc, cap| {
            if &cap[0] == "do()" {
                *acc = true;
                return Some(None);
            }
            if &cap[0] == "don't()" {
                *acc = false;
                return Some(None);
            }
            if *acc { Some(Some(cap)) } else { Some(None) }
        })
        .flatten()
        .map(|c| c[1].parse::<u64>().unwrap() * c[2].parse::<u64>().unwrap())
        .sum();
    println!("Part2: {s}");
}
