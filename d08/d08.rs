use std::collections::{HashMap, HashSet};
use vecmath::{vec2_add, vec2_sub};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Grid {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<[isize; 2]>>,
}

impl Grid {
    fn from_lines<'s>(lines: impl IntoIterator<Item = &'s str>) -> Self {
        let mut res = Grid::default();
        for (y, l) in lines
            .into_iter()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .enumerate()
        {
            res.width = l.len();
            res.height = y + 1;
            for (x, c) in l.chars().enumerate() {
                if c != '.' {
                    res.antennas
                        .entry(c)
                        .or_insert_with(Vec::default)
                        .push([x as isize, y as isize]);
                }
            }
        }
        res
    }
}

fn get_antinodes(grid: &Grid) -> HashSet<[isize; 2]> {
    let mut antinodes = HashSet::new();
    let is_inside = |[x, y]: [isize; 2]| {
        (0..grid.width as isize).contains(&x) && (0..grid.height as isize).contains(&y)
    };
    for ps in grid.antennas.values() {
        for i in 0..ps.len() - 1 {
            for j in i + 1..ps.len() {
                let a = ps[i];
                let b = ps[j];
                let v = vec2_sub(b, a);
                let a1 = vec2_sub(a, v);
                if is_inside(a1) {
                    antinodes.insert(a1);
                }
                let a2 = vec2_add(b, v);
                if is_inside(a2) {
                    antinodes.insert(a2);
                }
            }
        }
    }
    antinodes
}

fn get_all_antinodes(grid: &Grid) -> HashSet<[isize; 2]> {
    let mut antinodes = HashSet::new();
    let is_inside = |[x, y]: [isize; 2]| {
        (0..grid.width as isize).contains(&x) && (0..grid.height as isize).contains(&y)
    };
    for ps in grid.antennas.values() {
        for i in 0..ps.len() - 1 {
            for j in i + 1..ps.len() {
                let a = ps[i];
                let b = ps[j];
                let v = vec2_sub(b, a);
                let mut an = b;
                while is_inside(an) {
                    antinodes.insert(an);
                    an = vec2_add(an, v);
                }
                let mut an = a;
                while is_inside(an) {
                    antinodes.insert(an);
                    an = vec2_sub(an, v);
                }
            }
        }
    }
    antinodes
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let grid = Grid::from_lines(input.lines());
    let antinodes = get_antinodes(&grid);
    println!("Part1: {}", antinodes.len());
    let antinodes = get_all_antinodes(&grid);
    println!("Part2: {}", antinodes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let g = Grid::from_lines(["....", ".a..", "..aA", "x...", "...."]);
        assert_eq!(
            g,
            Grid {
                width: 4,
                height: 5,
                antennas: HashMap::from([
                    ('a', vec![[1, 1], [2, 2]]),
                    ('A', vec![[3, 2]]),
                    ('x', vec![[0, 3]])
                ]),
            }
        );
    }

    #[test]
    fn test_get_antinodes_1() {
        let grid = Grid::from_lines(
            r###"
                ..........
                ...#......
                ..........
                ....a.....
                ..........
                .....a....
                ..........
                ......#...
                ..........
                ..........
            "###
            .lines(),
        );
        let antinodes = get_antinodes(&grid);
        assert_eq!(antinodes, HashSet::from([[3, 1], [6, 7]]));
    }
}
