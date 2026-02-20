use aoc2016::graph::{NoPathFound, a_star_rev, bfs};
use aoc2016::grid::{Grid, Pos};
use aoc2016::math::vec2_hamming;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum CountingCell {
    #[default]
    Clear,
    Path(usize),
    Wall,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct DistancesToGoal {
    grid: Grid<CountingCell>,
    path: Vec<Pos>,
    start: Pos,
    goal: Pos,
}

fn into_distances_to_goal(
    grid: Grid<Cell>,
    start: Pos,
    goal: Pos,
) -> Result<DistancesToGoal, NoPathFound> {
    let path: Vec<_> = find_shortest_path_rev(&grid, start, goal)?.collect();

    let mut grid = grid.map(|c| match c {
        Cell::Clear => CountingCell::Clear,
        Cell::Wall => CountingCell::Wall,
    });

    for (dist, &p) in path.iter().enumerate() {
        grid[p] = CountingCell::Path(dist);
    }

    Ok(DistancesToGoal {
        grid,
        start,
        goal,
        path,
    })
}

struct CheatResult {
    best_pos: Pos,
    best_diff: usize,
    count_at_least_100: usize,
}

fn find_best_cheat_pos(grid: &Grid<CountingCell>) -> Result<CheatResult, NoPathFound> {
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

fn find_cheats_at_least(
    DistancesToGoal { grid, path, .. }: &DistancesToGoal,
    max_cheat: usize,
    min_improvement: usize,
) -> HashSet<(Pos, Pos)> {
    #[derive(Copy, Clone)]
    struct Node(Pos, usize);

    impl Hash for Node {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state)
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for Node {}

    let mut valid_cheats = HashSet::new();
    for &pos in path {
        valid_cheats.extend(
            bfs(
                Node(pos, 0),
                |&Node(n, _)| n != pos && matches!(grid[n], CountingCell::Path(_)),
                |&Node([x, y], c)| {
                    [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]]
                        .into_iter()
                        .filter(move |_| c < max_cheat)
                        .filter(|&p| {
                            grid.is_inside(p)
                                && matches!(grid[p], CountingCell::Wall | CountingCell::Path(_))
                        })
                        .map(move |p| Node(p, c + 1))
                },
            )
            .filter_map(|Node(p, c)| match (grid[pos], grid[p]) {
                (CountingCell::Path(a), CountingCell::Path(b))
                    if c > 1 && a.abs_diff(b) >= min_improvement + c =>
                {
                    if a < b {
                        Some((p, pos))
                    } else {
                        Some((pos, p))
                    }
                }
                _ => None,
            }),
        );
    }

    valid_cheats
}

fn main() {
    let input = include_str!("sample.txt");
    // let input = include_str!("input.txt");
    let Input { grid, start, goal } = input.parse().unwrap();
    let distances = into_distances_to_goal(grid, start, goal).unwrap();

    // Part 1
    let t0 = Instant::now();
    let CheatResult {
        best_pos,
        best_diff,
        count_at_least_100,
    } = find_best_cheat_pos(&distances.grid).unwrap();
    println!(
        "Part1: {}\nd = {} at {:?} (took {:?})",
        count_at_least_100,
        best_diff,
        best_pos,
        t0.elapsed()
    );

    // Part 2
    let t0 = Instant::now();
    let cheats = find_cheats_at_least(&distances, 20, 72);
    println!("{:?} (took {:?})", cheats.len(), t0.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cheats() {
        let Input { grid, start, goal } = r"
            S##E
            .#..
            ....
        "
        .parse()
        .unwrap();
        let distances = into_distances_to_goal(grid, start, goal).unwrap();
        println!("{:?}", distances.path);
        let c = find_cheats_at_least(&distances, 20, 1);
        println!("{:?}", c);
    }
}
