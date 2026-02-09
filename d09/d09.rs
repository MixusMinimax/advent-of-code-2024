#![feature(assert_matches)]

use std::assert_matches;
use std::fmt::{Display, Formatter, Write};
use std::iter::repeat_n;

type ID = u32;
#[derive(Clone)]
struct FS(Vec<Option<ID>>);

fn parse_fs(s: &str) -> FS {
    FS(s.trim()
        .chars()
        .scan((true, 0), |(is_file, next_id), c| {
            assert_matches!(c, '0'..='9');
            let r = if *is_file {
                let r = repeat_n(Some(*next_id), (c as u8 - b'0') as usize);
                *next_id += 1;
                r
            } else {
                repeat_n(None, (c as u8 - b'0') as usize)
            };
            *is_file ^= true;
            Some(r)
        })
        .flatten()
        .collect())
}

impl Display for FS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter().copied() {
            f.write_char(if let Some(id) = c {
                assert!(id < 10);
                (id as u8 + b'0') as char
            } else {
                '.'
            })?;
        }
        Ok(())
    }
}

fn compact_fs(FS(mut fs): FS) -> FS {
    let mut a = 0;
    let mut b = fs.len() - 1;
    loop {
        while a < b && fs[a].is_some() {
            a += 1;
        }
        while a < b && fs[b].is_none() {
            b -= 1;
        }
        if a < b && fs[a].is_none() && fs[b].is_some() {
            fs.swap(a, b);
        } else {
            break;
        }
    }
    FS(fs)
}

fn compact_fs_non_fragmented(FS(mut fs): FS) -> FS {
    let mut a = 0;
    let mut b = fs.len();
    'outer: loop {
        loop {
            b -= 1;
            if b == 0 {
                break 'outer;
            }
            if fs[b].is_some() {
                break;
            }
        }
        let id = fs[b].unwrap();
        let mut size = 1;
        while b > a
            && let Some(i) = fs[b - 1]
            && id == i
        {
            b -= 1;
            size += 1;
        }
        while fs[a].is_some() {
            a += 1;
            if a >= b {
                break 'outer;
            }
        }
        let mut target = 0;
        let mut found = false;
        let mut hole_size = 0;
        'find: for (i, x) in fs.iter().enumerate().take(b).skip(a) {
            if x.is_some() {
                hole_size = 0;
            } else {
                if hole_size == 0 {
                    target = i;
                }
                hole_size += 1;
                if hole_size == size {
                    found = true;
                    break 'find;
                }
            }
        }
        if found {
            fs[b..b + size].iter_mut().for_each(|f| *f = None);
            fs[target..target + size]
                .iter_mut()
                .for_each(|f| *f = Some(id));
        }
    }
    FS(fs)
}

fn checksum(fs: &FS) -> u64 {
    fs.0.iter()
        .copied()
        .enumerate()
        .flat_map(|(i, c)| c.map(|c| (i, c)))
        .fold(0, |acc, (i, c)| acc + i as u64 * c as u64)
}

fn main() {
    let input = include_str!("input.txt");
    let fs = parse_fs(input);
    let compacted = compact_fs(fs.clone());
    let cks = checksum(&compacted);
    println!("Part1: {}", cks);
    let compacted = compact_fs_non_fragmented(fs);
    let cks = checksum(&compacted);
    println!("Part2: {}", cks);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_fs_expanded(s: &str) -> FS {
        FS(s.trim()
            .chars()
            .map(|c| match c {
                '0'..='9' => Some((c as u8 - b'0') as _),
                '.' => None,
                _ => panic!("no."),
            })
            .collect())
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_fs("12345").0,
            [
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2),
            ]
        );
    }

    #[test]
    fn test_compact_1() {
        let fs = parse_fs("12345");
        assert_eq!(fs.to_string(), "0..111....22222");
        let fs = compact_fs(fs);
        assert_eq!(fs.to_string(), "022111222......");
    }

    #[test]
    fn test_compact_2() {
        let fs = parse_fs("2333133121414131402");
        assert_eq!(fs.to_string(), "00...111...2...333.44.5555.6666.777.888899");
        let fs = compact_fs(fs);
        assert_eq!(fs.to_string(), "0099811188827773336446555566..............");
    }

    #[test]
    fn test_checksum() {
        let fs = compact_fs(parse_fs("2333133121414131402"));
        assert_eq!(checksum(&fs), 1928);
    }

    #[test]
    fn test_parse_expanded() {
        let fs = parse_fs_expanded("00...111...2...333.44.5555.6666.777.888899");
        let s = fs.to_string();
        assert_eq!(s, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_compact_fs_non_fragmented() {
        let fs = parse_fs_expanded("00...111...2...333.44.5555.6666.777.888899");
        let compacted = compact_fs_non_fragmented(fs);
        assert_eq!(
            compacted.to_string(),
            "00992111777.44.333....5555.6666.....8888.."
        );
        assert_eq!(checksum(&compacted), 2858);
    }
}
