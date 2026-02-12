use aoc2016::graph::{NoPathFound, a_star_rev};
use aoc2016::grid::{Grid, Pos};
use std::fmt::{Display, Formatter, Write};
use std::iter::once;
use std::str::FromStr;
use vecmath::{vec2_add, vec2_dot};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
enum Cell {
    #[default]
    Clear,
    Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Angle {
    Zero,
    Quarter,
    Half,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Field {
    grid: Grid<Cell>,
    start: Pos,
    goal: Pos,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut goal = None;
        let grid = Grid::try_from_lines(
            s.lines().map(str::trim).filter(|s| !s.is_empty()),
            |p, c| {
                Ok(match c {
                    '.' => Cell::Clear,
                    '#' => Cell::Wall,
                    'S' => {
                        start = Some(p);
                        Cell::Clear
                    }
                    'E' => {
                        goal = Some(p);
                        Cell::Clear
                    }
                    _ => return Err(()),
                })
            },
        )?;
        Ok(Field {
            grid,
            start: start.ok_or(())?,
            goal: goal.ok_or(())?,
        })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.grid
            .display_pos(|pos, c, f| {
                f.write_char({
                    if pos == self.start {
                        'S'
                    } else if pos == self.goal {
                        'E'
                    } else {
                        match c {
                            Cell::Clear => '.',
                            Cell::Wall => '#',
                        }
                    }
                })
            })
            .fmt(f)
    }
}

impl Dir {
    fn vec(&self) -> Pos {
        match self {
            Dir::North => [0, -1],
            Dir::East => [1, 0],
            Dir::South => [0, 1],
            Dir::West => [-1, 0],
        }
    }

    fn abs_angle(&self, other: &Dir) -> Angle {
        match vec2_dot(self.vec(), other.vec()) {
            1 => Angle::Zero,
            0 => Angle::Quarter,
            -1 => Angle::Half,
            _ => unreachable!(),
        }
    }
}

impl Angle {
    fn cost(&self) -> i64 {
        match self {
            Angle::Zero => 0,
            Angle::Quarter => 1000,
            Angle::Half => 2000,
        }
    }
}

fn find_cheapest_path(field: &Field) -> Result<(Vec<Pos>, i64), NoPathFound> {
    let (path, last) = a_star_rev(
        &(field.start, Dir::East),
        |&(p, _)| p == field.goal,
        |&(p, dir)| {
            [Dir::North, Dir::East, Dir::South, Dir::West]
                .into_iter()
                .filter_map(move |n_dir| {
                    let n_pos = vec2_add(p, n_dir.vec());
                    if !field.grid.is_inside(n_pos) || field.grid[n_pos] == Cell::Wall {
                        return None;
                    }
                    Some(((n_pos, n_dir), dir.abs_angle(&n_dir)))
                })
        },
        |_| 0,
        |_, angle, _| angle.cost() + 1,
    )?;
    let total_cost = path.iter().map(|(_, angle)| angle.cost() + 1).sum();
    Ok((
        path.into_iter()
            .rev()
            .map(|((p, _), _)| p)
            .chain(once(last.0))
            .collect(),
        total_cost,
    ))
}

fn main() {
    let input = include_str!("input.txt");
    let f = input.parse().unwrap();
    let (p, c) = find_cheapest_path(&f).unwrap();
    println!("Part1: {} steps, score: {}", p.len() - 1, c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let f: Field = {
            r"
            #######
            #...#E#
            #S#...#
            #######
            "
            .parse()
            .unwrap()
        };
        assert_eq!(
            f.to_string(),
            "\
            #######\n\
            #...#E#\n\
            #S#...#\n\
            #######"
        );
    }

    #[test]
    fn test_dir() {
        assert_eq!(Dir::West.abs_angle(&Dir::West), Angle::Zero);
        assert_eq!(Dir::North.abs_angle(&Dir::East), Angle::Quarter);
        assert_eq!(Dir::East.abs_angle(&Dir::West), Angle::Half);
        assert_eq!(Dir::West.abs_angle(&Dir::South), Angle::Quarter);
    }

    #[test]
    fn test_find_cheapest_path_1() {
        let f: Field = {
            r"
            #######
            #...#E#
            #S#...#
            #######
            "
            .parse()
            .unwrap()
        };
        let r = find_cheapest_path(&f);
        assert!(r.is_ok());
        let (p, c) = r.unwrap();
        assert_eq!(
            p,
            [
                [1, 2],
                [1, 1],
                [2, 1],
                [3, 1],
                [3, 2],
                [4, 2],
                [5, 2],
                [5, 1],
            ],
        );
        assert_eq!(c, 7 + 5000);
    }

    #[test]
    fn test_sample() {
        let f: Field = include_str!("sample.txt").parse().unwrap();
        let r = find_cheapest_path(&f);
        assert!(r.is_ok());
        let (p, c) = r.unwrap();
        assert_eq!(p.len(), 37);
        assert_eq!(c, 7036);
    }
}
