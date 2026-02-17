use std::collections::HashMap;
use std::time::Instant;

struct Input<'a>(Vec<&'a [u8]>, Vec<&'a [u8]>);

fn parse_input(input: &'_ str) -> Option<Input<'_>> {
    let mut it = input
        .trim()
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let rules = it
        .next()?
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::as_bytes)
        .collect();
    Some(Input(rules, it.map(str::as_bytes).collect()))
}

fn count_possible_compositions<'i>(
    words: &[&'i [u8]],
    mut tokens: Vec<&[u8]>,
) -> Vec<(&'i [u8], usize)> {
    fn descend<'a>(
        input: &'a [u8],
        tokens: &HashMap<u8, &[&'a [u8]]>,
        cache: &mut HashMap<&'a [u8], usize>,
    ) -> usize {
        if let Some(&cached) = cache.get(input) {
            return cached;
        }
        if input.is_empty() {
            return 1;
        }
        let result = tokens
            .get(&input[0])
            .copied()
            .unwrap_or_default()
            .iter()
            .map(|token| {
                if let Some(stripped) = input[1..].strip_prefix(&token[1..]) {
                    descend(stripped, tokens, cache)
                } else {
                    0
                }
            })
            .sum();
        cache.insert(input, result);
        result
    }

    tokens.sort_unstable_by_key(|w| w[0]);

    let tokens = tokens
        .chunk_by(|a, b| a[0] == b[0])
        .map(|w| (w[0][0], w))
        .collect();

    let mut cache = HashMap::new();

    words
        .iter()
        .map(|&word| (word, descend(word, &tokens, &mut cache)))
        .collect()
}

fn main() {
    let input = include_str!("input.txt");
    let Input(tokens, words) = parse_input(input).unwrap();
    let start = Instant::now();
    let c = count_possible_compositions(&words, tokens);
    println!("{:?}", start.elapsed());
    println!("Part1: {}", c.iter().filter(|(_, c)| *c != 0).count());
    println!("Part2: {}", c.iter().map(|(_, c)| *c).sum::<usize>());
}
