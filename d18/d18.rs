use aoc2016::graph::{NoPathFound, a_star_rev};
use aoc2016::grid::{Grid, Pos};
use std::iter::once;

fn parse_coords(s: &str) -> impl Iterator<Item = Pos> {
    s.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            let mut it = l.split(",");
            Some([it.next()?.parse().ok()?, it.next()?.parse().ok()?])
        })
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
enum Cell {
    #[default]
    Clear,
    Byte,
}

impl Cell {
    #[allow(dead_code)]
    fn into_char(self) -> char {
        match self {
            Cell::Clear => '.',
            Cell::Byte => '#',
        }
    }
}

fn find_path(grid: &Grid<Cell>, start: Pos, goal: Pos) -> Result<Vec<Pos>, NoPathFound> {
    fn hamming([ax, ay]: Pos, [bx, by]: Pos) -> usize {
        ax.abs_diff(bx) + ay.abs_diff(by)
    }
    Ok(a_star_rev(
        &start,
        |&n| n == goal,
        |&[x, y]| {
            [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]]
                .into_iter()
                .filter(|&pos| grid.is_inside(pos) && grid[pos] != Cell::Byte)
                .map(|n| (n, ()))
        },
        |&n| hamming(n, goal) as i64,
        |&a, (), &b| hamming(a, b) as i64,
    )?
    .0
    .into_iter()
    .rev()
    .map(|(p, _)| p)
    .chain(once(goal))
    .collect())
}

fn find_first_blocking_byte(grid: &Grid<Cell>, coords: &[Pos], start: Pos, goal: Pos) -> Pos {
    let mut begin = 0;
    let mut end = coords.len();
    while begin + 1 < end {
        let current = (begin + end - 1) / 2;
        let mut grid = grid.clone();
        coords
            .iter()
            .copied()
            .take(current)
            .for_each(|p| grid[p] = Cell::Byte);

        if find_path(&grid, start, goal).is_ok() {
            begin = current + 1;
        } else {
            end = current + 1;
        }
    }
    coords[begin - 1]
}

fn main() {
    let input = include_str!("input.txt");
    let coords = parse_coords(input).collect::<Vec<_>>();
    let mut grid = Grid::<Cell>::new([71, 71]);
    let grid2 = grid.clone();
    coords
        .iter()
        .copied()
        .take(1024)
        .for_each(|p| grid[p] = Cell::Byte);
    let path = find_path(&grid, [0, 0], [70, 70]).unwrap();
    let steps = path.len() - 1;
    println!("Part1: {}", steps);

    let result = find_first_blocking_byte(&grid2, &coords, [0, 0], [70, 70]);
    println!("Part2: {},{}", result[0], result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let coords: Vec<_> = parse_coords(
            r"
            5,4
            4,2
            4,5
            3,0
            2,1
            ",
        )
        .collect();
        assert_eq!(coords, [[5, 4], [4, 2], [4, 5], [3, 0], [2, 1]]);
    }

    #[test]
    fn test_find_path() {
        let mut grid = Grid::<Cell>::new([7, 7]);
        parse_coords(include_str!("sample.txt"))
            .take(12)
            .for_each(|p| grid[p] = Cell::Byte);
        let mut result = grid.clone().map(Cell::into_char);
        let path = find_path(&grid, [0, 0], [6, 6]).unwrap();
        path.into_iter().for_each(|p| result[p] = 'O');
        println!("{}", result);
    }

    #[test]
    fn test_find_first_blocking_byte() {
        let grid = Grid::<Cell>::new([7, 7]);
        let coords = parse_coords(include_str!("sample.txt")).collect::<Vec<_>>();
        let result = find_first_blocking_byte(&grid, &coords, [0, 0], [6, 6]);
        assert_eq!(result, [6, 1]);
    }
}
