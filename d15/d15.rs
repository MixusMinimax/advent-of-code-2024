use aoc2016::grid::{Grid, Pos};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Cell {
    Clear,
    Box,
    Wall,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Field {
    grid: Grid<Cell>,
    dirs: Vec<Dir>,
    pos: Pos,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Clear => '.',
            Cell::Box => 'O',
            Cell::Wall => '#',
        })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.grid
            .display_pos(|p, c, f| {
                if p == self.pos {
                    f.write_char('@')
                } else {
                    Display::fmt(c, f)
                }
            })
            .fmt(f)
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Field, Self::Err> {
        let mut it = s.lines().map(str::trim);
        let mut pos = Err(());
        let grid = Grid::try_from_lines(
            it.by_ref()
                .skip_while(|s| s.is_empty())
                .take_while(|s| !s.is_empty()),
            |p, c| {
                Ok(match c {
                    '.' => Cell::Clear,
                    'O' => Cell::Box,
                    '#' => Cell::Wall,
                    '@' => {
                        pos = Ok(p);
                        Cell::Clear
                    }
                    _ => return Err(()),
                })
            },
        )?;
        let dirs = it
            .flat_map(str::chars)
            .map(|c| {
                Ok(match c {
                    '^' => Dir::Up,
                    '>' => Dir::Right,
                    'v' => Dir::Down,
                    '<' => Dir::Left,
                    _ => return Err(()),
                })
            })
            .collect::<Result<_, _>>()?;
        let pos = pos?;
        Ok(Field { grid, dirs, pos })
    }
}

fn main() {
    let input = include_str!("sample.txt");
    let f: Field = input.parse().unwrap();
    println!("{f}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let f = Field::from_str(
            r#"

            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<

            "#,
        );
        assert!(f.is_ok());
        let Field { grid, dirs, pos } = f.unwrap();
        assert_eq!(grid.size, [8, 8]);
        assert_eq!(grid.cells[0..8], [Cell::Wall; 8]);
        assert_eq!(grid.rows().nth(7).unwrap(), [Cell::Wall; 8]);
        assert_eq!(
            grid.rows().nth(2).unwrap(),
            [
                Cell::Wall,
                Cell::Wall,
                Cell::Clear,
                Cell::Clear,
                Cell::Box,
                Cell::Clear,
                Cell::Clear,
                Cell::Wall,
            ],
        );
        assert_eq!(
            dirs[5..10],
            [Dir::Right, Dir::Down, Dir::Down, Dir::Left, Dir::Down],
        );
        assert_eq!(pos, [2, 2]);
    }
}
