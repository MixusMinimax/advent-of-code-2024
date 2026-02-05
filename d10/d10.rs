use aoc2016::grid::{Grid, Pos};
use std::collections::HashSet;

fn reachable_peaks(
    grid: &Grid<u8>,
    cache: &mut Grid<Option<HashSet<Pos>>>,
    p @ [x, y]: Pos,
) -> usize {
    if let Some(ref c) = cache[p] {
        return c.len();
    }
    let v = grid[p];
    if !(0..=9).contains(&v) {
        cache[p] = Some(HashSet::new());
        return 0;
    }
    if v == 9 {
        cache[p] = Some(HashSet::from([p]));
        return 1;
    }
    let mut new_cache = HashSet::new();
    for n in [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]] {
        if grid.is_inside(n) && grid[n] == v + 1 {
            reachable_peaks(grid, cache, n);
            if let Some(ref c) = cache[n] {
                new_cache.extend(c);
            }
        }
    }
    let l = new_cache.len();
    cache[p] = Some(new_cache);
    l
}

fn trailheads(grid: &Grid<u8>) -> (usize, Grid<usize>) {
    let mut total = 0;
    let mut cache = Grid::new(grid.size);
    for i in 0..grid.len() {
        if grid[i] == 0 {
            total += reachable_peaks(
                grid,
                &mut cache,
                [(i % grid.width()) as isize, (i / grid.height()) as isize],
            );
        }
    }
    (total, cache.map(|c| c.map(|c| c.len()).unwrap_or_default()))
}

fn trailhead_ratings(grid: &Grid<u8>) -> (usize, Grid<usize>) {
    fn paths(grid: &Grid<u8>, cache: &mut Grid<Option<usize>>, p @ [x, y]: Pos) -> usize {
        if let Some(c) = cache[p] {
            return c;
        }
        let v = grid[p];
        if !(0..=9).contains(&v) {
            cache[p] = Some(0);
            return 0;
        }
        if v == 9 {
            cache[p] = Some(1);
            return 1;
        }
        let mut total = 0;
        for n in [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]] {
            if grid.is_inside(n) && grid[n] == v + 1 {
                total += paths(grid, cache, n);
            }
        }
        cache[p] = Some(total);
        total
    }

    let mut total = 0;
    let mut cache = Grid::new(grid.size);
    for i in 0..grid.len() {
        if grid[i] == 0 {
            total += paths(
                grid,
                &mut cache,
                [(i % grid.width()) as isize, (i / grid.height()) as isize],
            );
        }
    }
    (total, cache.map(Option::unwrap_or_default))
}

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::from_lines(input.lines(), |_, c| c as u8 - b'0');
    let (total, _) = trailheads(&grid);
    println!("Part1: {}", total);
    let (total, _) = trailhead_ratings(&grid);
    println!("Part2: {}", total);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    macro_rules! grid {
        ($s:expr) => {
            Grid::from_lines(
                $s.lines(),
                |_, c| if c == '.' { 10 } else { c as u8 - b'0' },
            )
        };
    }

    #[test]
    fn test_trailheads_1() {
        let grid = grid!(
            r#"
            0123
            1234
            8765
            9876
            "#
        );
        let (total, th) = trailheads(&grid);
        assert_eq!(total, 1);
        assert_eq!(th.to_string(), "1111\n1111\n1111\n1111");
    }

    #[test]
    fn test_trailheads_2() {
        let grid = grid!(
            r#"
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
            "#
        );
        let (total, th) = trailheads(&grid);
        assert_eq!(total, 2);
        assert_eq!(
            th.to_string(),
            "0002000\n0002000\n0002000\n1112111\n1000001\n1000001\n1000001",
        );
    }

    #[test]
    fn test_trailheads_3() {
        let grid = grid!(
            r#"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
            "#
        );
        let (total, th) = trailheads(&grid);
        assert_eq!(total, 36);
        assert_eq!(
            th.to_string(),
            "01556333\n01555113\n12555133\n12551333\n55333133\n55111333\n55111133\n05111100",
        );
    }

    #[test]
    fn test_trailhead_ratings_1() {
        let grid = grid!(
            r#"
            .....0.
            ..4321.
            ..5..2.
            ..6543.
            ..7..4.
            ..8765.
            ..9....
            "#
        );
        let (total, thr) = trailhead_ratings(&grid);
        assert_eq!(total, 3);
        assert_eq!(
            thr.to_string(),
            "0000030\n0011130\n0010020\n0011120\n0010010\n0011110\n0010000",
        );
    }

    #[test]
    fn test_trailhead_ratings_2() {
        let grid = grid!(
            r#"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
            "#
        );
        let (total, _) = trailhead_ratings(&grid);
        assert_eq!(total, 81);
    }
}
