use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
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
struct Field {
    cells: aoc2016::grid::Grid<Cell>,
    pos: [isize; 2],
    heading: u8,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "pos: [{}, {}]", self.pos[0], self.pos[1])?;
        write!(
            f,
            "{}",
            self.cells.display_pos(|pos, cell, f| {
                if pos == self.pos {
                    match self.heading {
                        0 => f.write_char('^'),
                        1 => f.write_char('>'),
                        2 => f.write_char('v'),
                        3 => f.write_char('<'),
                        _ => unreachable!(),
                    }
                } else {
                    write!(f, "{}", cell)
                }
            })
        )
    }
}

impl Field {
    fn from_lines<'s>(lines: impl IntoIterator<Item = &'s str>) -> Self {
        let mut pos = [0, 0];
        let cells = aoc2016::grid::Grid::from_lines(lines, |p, c| match c {
            '.' => Cell::Clear,
            '#' => Cell::Wall,
            '^' => {
                pos = p;
                Cell::Clear
            }
            _ => panic!("unexpected {c}"),
        });
        Self {
            cells,
            heading: 0,
            pos,
        }
    }
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

impl Field {
    fn step(mut self) -> Self {
        let pos = self.pos;
        self.cells[pos] = Cell::Visited;
        for h in self.heading..self.heading + 4 {
            let h = h % 4;
            let next = vec2_add(pos, dir_vec(h));
            if !self.cells.is_inside(next) || self.cells[next] != Cell::Wall {
                self.heading = h;
                self.pos = next;
                return self;
            }
        }
        panic!("Stuck!");
    }
}

fn causes_loop(mut field: Field) -> bool {
    let mut visited = HashSet::new();
    while field.cells.is_inside(field.pos) {
        visited.insert((field.pos, field.heading));
        field = field.step();
        if visited.contains(&(field.pos, field.heading)) {
            return true;
        }
    }
    false
}

fn calc_obstructions(field: &Field) -> Vec<[isize; 2]> {
    let mut obstructions = Vec::new();
    for y in 0..field.cells.height() as isize {
        for x in 0..field.cells.width() as isize {
            if field.cells[[x, y]] == Cell::Visited && [x, y] != field.pos {
                let mut grid = field.clone();
                grid.cells[[x, y]] = Cell::Wall;
                if causes_loop(grid) {
                    obstructions.push([x, y]);
                }
            }
        }
    }
    obstructions
}

fn part1(mut field: Field) -> Field {
    while field.cells.is_inside(field.pos) {
        field = field.step();
    }
    // println!("{grid}");
    let visit_count = field
        .cells
        .cells
        .iter()
        .filter(|c| matches!(c, Cell::Visited))
        .count();
    println!("Part1: {visit_count}");
    field
}

fn part2(field: &Field) {
    let obstacles = calc_obstructions(field);
    println!("Part2: {}", obstacles.len());
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let field = Field::from_lines(input.lines());

    let pos = field.pos;
    let mut grid = part1(field.clone());
    grid.pos = pos;
    grid.heading = 0;
    part2(&grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop() {
        let mut field = Field::from_lines(
            r##"
            .#....
            .....#
            ......
            .^....
            ....#.
            "##
            .lines(),
        );
        while field.cells.is_inside(field.pos) {
            field = field.step();
        }
        println!("{field}");
        field.pos = [1, 3];
        field.heading = 0;
        println!("\n{field}");
        let obs = calc_obstructions(&field);
        println!("{obs:?}");
    }
}
