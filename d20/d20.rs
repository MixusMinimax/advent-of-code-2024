use aoc2016::graph::{NoPathFound, a_star_rev};
use aoc2016::grid::{Grid, Pos};
use aoc2016::math::vec2_hamming;
use std::fmt::{Display, Formatter, Write};
use std::iter::once;
use std::str::FromStr;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Cell {
    #[default]
    Clear,
    Wall,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Input {
    grid: Grid<Cell>,
    start: Pos,
    goal: Pos,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut goal = None;
        let grid = Grid::try_from_lines(s.lines(), |p, c| {
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
        });
        Ok(Input {
            grid: grid?,
            start: start.ok_or(())?,
            goal: goal.ok_or(())?,
        })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.grid
            .display_pos(|pos, c, f| {
                f.write_char(if pos == self.start {
                    'S'
                } else if pos == self.goal {
                    'E'
                } else {
                    match c {
                        Cell::Clear => '.',
                        Cell::Wall => '#',
                    }
                })
            })
            .fmt(f)
    }
}

fn find_shortest_path_rev(
    grid: &Grid<Cell>,
    start: Pos,
    goal: Pos,
) -> Result<impl Iterator<Item = Pos> + 'static, NoPathFound> {
    let (path, _) = a_star_rev(
        &start,
        |&n| n == goal,
        |&[x, y]| {
            [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]]
                .into_iter()
                .filter(|&n2| grid.is_inside(n2) && grid[n2] != Cell::Wall)
                .map(|n2| (n2, ()))
        },
        |&n| vec2_hamming(n, goal) as i64,
        |&a, (), &b| vec2_hamming(a, b) as i64,
    )?;
    Ok(once(goal).chain(path.into_iter().map(|(n, _)| n)))
}

struct CheatResult {
    best_pos: Pos,
    best_diff: usize,
    count_at_least_100: usize,
}

fn find_best_cheat_pos(
    grid: Grid<Cell>,
    start: Pos,
    goal: Pos,
) -> Result<CheatResult, NoPathFound> {
    let path = find_shortest_path_rev(&grid, start, goal)?;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    enum CountingCell {
        #[default]
        Clear,
        Path(usize),
        Wall,
    }

    let mut grid = grid.map(|c| match c {
        Cell::Clear => CountingCell::Clear,
        Cell::Wall => CountingCell::Wall,
    });

    for (dist, p) in path.enumerate() {
        grid[p] = CountingCell::Path(dist);
    }

    let mut best_diff = 0;
    let mut best_pos = [0, 0];
    let mut count_at_least_100 = 0;

    for p @ [x, y] in grid.positions() {
        let neighbors = [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]];
        for (i, a) in neighbors.iter().copied().enumerate().take(3) {
            for b in neighbors.iter().copied().skip(i + 1) {
                if !grid.is_inside(a) || !grid.is_inside(b) {
                    continue;
                }
                if let (CountingCell::Path(a_dist), CountingCell::Path(b_dist)) = (grid[a], grid[b])
                {
                    let diff = a_dist.abs_diff(b_dist) - 2;
                    if diff >= 100 {
                        count_at_least_100 += 1;
                    }
                    if diff > best_diff {
                        best_pos = p;
                        best_diff = diff;
                    }
                }
            }
        }
    }

    Ok(CheatResult {
        best_pos,
        best_diff,
        count_at_least_100,
    })
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let Input { grid, start, goal } = input.parse().unwrap();
    let t0 = Instant::now();
    let CheatResult {
        best_pos,
        best_diff,
        count_at_least_100,
    } = find_best_cheat_pos(grid, start, goal).unwrap();
    println!(
        "Part1: {}\nd = {} at {:?} (took {:?})",
        count_at_least_100,
        best_diff,
        best_pos,
        t0.elapsed()
    );
}
