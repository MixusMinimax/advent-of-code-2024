use std::cell::OnceCell;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Index, IndexMut};
use std::rc::Rc;
use vecmath::vec2_add;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
enum Cell {
    #[default]
    Clear,
    Visited,
    Wall,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Clear => f.write_char('.'),
            Cell::Visited => f.write_char('X'),
            Cell::Wall => f.write_char('#'),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Grid {
    cells: Vec<Cell>,
    size: [usize; 2],
    pos: [isize; 2],
    heading: u8,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos: [{}, {}]", self.pos[0], self.pos[1])?;
        for (i, cell) in self.cells.iter().enumerate() {
            let x = (i % self.size[0]) as isize;
            let y = (i / self.size[0]) as isize;
            if x == 0 {
                writeln!(f)?;
            }
            if [x, y] == self.pos {
                match self.heading {
                    0 => f.write_char('^')?,
                    1 => f.write_char('>')?,
                    2 => f.write_char('v')?,
                    3 => f.write_char('<')?,
                    _ => unreachable!(),
                }
            } else {
                write!(f, "{}", cell)?;
            }
        }
        Ok(())
    }
}

impl Grid {
    fn from_lines<'s>(lines: impl IntoIterator<Item = &'s str>) -> Self {
        let mut width = None;
        let pos = Rc::<OnceCell<_>>::default();

        let cells: Vec<_> = lines
            .into_iter()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                let l = line.len();
                if let Some(width) = width {
                    assert_eq!(width, l);
                } else {
                    width = Some(l);
                }
                let pos = pos.clone();
                line.chars().enumerate().map(move |(x, c)| match c {
                    '.' => Cell::Clear,
                    '#' => Cell::Wall,
                    '^' => {
                        pos.set([x as isize, y as isize]).unwrap();
                        Cell::Clear
                    }
                    _ => panic!("unexpected {c}"),
                })
            })
            .collect();
        let width = width.unwrap();
        let height = cells.len() / width;
        Self {
            cells,
            size: [width, height],
            pos: Rc::into_inner(pos).unwrap().into_inner().unwrap(),
            heading: 0,
        }
    }
}

fn idx([x, y]: [isize; 2], [width, height]: [usize; 2]) -> usize {
    assert!((0..width as isize).contains(&x));
    assert!((0..height as isize).contains(&y));
    y as usize * width + x as usize
}

fn dir_vec(heading: u8) -> [isize; 2] {
    match heading {
        0 => [0, -1],
        1 => [1, 0],
        2 => [0, 1],
        3 => [-1, 0],
        _ => panic!(),
    }
}

impl Index<[isize; 2]> for Grid {
    type Output = Cell;

    fn index(&self, index: [isize; 2]) -> &Self::Output {
        let i = idx(index, self.size);
        &self.cells[i]
    }
}

impl IndexMut<[isize; 2]> for Grid {
    fn index_mut(&mut self, index: [isize; 2]) -> &mut Self::Output {
        let i = idx(index, self.size);
        &mut self.cells[i]
    }
}

impl Grid {
    fn contains(&self, pos: [isize; 2]) -> bool {
        (0..self.size[0] as isize).contains(&pos[0]) && (0..self.size[1] as isize).contains(&pos[1])
    }

    fn step(mut self) -> Self {
        let pos = self.pos;
        self[pos] = Cell::Visited;
        for h in self.heading..self.heading + 4 {
            let h = h % 4;
            let next = vec2_add(pos, dir_vec(h));
            if !self.contains(next) || self[next] != Cell::Wall {
                self.heading = h;
                self.pos = next;
                return self;
            }
        }
        panic!("Stuck!");
    }
}

fn causes_loop(mut grid: Grid) -> bool {
    let mut visited = HashSet::new();
    while grid.contains(grid.pos) {
        visited.insert((grid.pos, grid.heading));
        grid = grid.step();
        if visited.contains(&(grid.pos, grid.heading)) {
            return true;
        }
    }
    false
}

fn calc_obstructions(grid: &Grid) -> Vec<[isize; 2]> {
    let mut obstructions = Vec::new();
    for y in 0..grid.size[1] as isize {
        for x in 0..grid.size[0] as isize {
            if grid[[x, y]] == Cell::Visited && [x, y] != grid.pos {
                let mut grid = grid.clone();
                grid[[x, y]] = Cell::Wall;
                if causes_loop(grid) {
                    obstructions.push([x, y]);
                }
            }
        }
    }
    obstructions
}

fn part1(mut grid: Grid) -> Grid {
    while grid.contains(grid.pos) {
        grid = grid.step();
    }
    // println!("{grid}");
    let visit_count = grid
        .cells
        .iter()
        .filter(|c| matches!(c, Cell::Visited))
        .count();
    println!("Part1: {visit_count}");
    grid
}

fn part2(grid: &Grid) {
    let obstacles = calc_obstructions(grid);
    println!("Part2: {}", obstacles.len());
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let grid = Grid::from_lines(input.lines());

    let pos = grid.pos;
    let mut grid = part1(grid.clone());
    grid.pos = pos;
    grid.heading = 0;
    part2(&grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop() {
        let mut grid = Grid::from_lines(
            r##"
            .#....
            .....#
            ......
            .^....
            ....#.
            "##
            .lines(),
        );
        while grid.contains(grid.pos) {
            grid = grid.step();
        }
        println!("{grid}");
        grid.pos = [1, 3];
        grid.heading = 0;
        println!("\n{grid}");
        let obs = calc_obstructions(&grid);
        println!("{obs:?}");
    }
}
