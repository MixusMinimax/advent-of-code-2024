use aoc2016::grid::{Grid, Pos};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use vecmath::vec2_add;

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
    pos: Pos,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Input(Field, Vec<Dir>);

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Clear => '.',
            Cell::Box => 'O',
            Cell::Wall => '#',
        })
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Dir::Up => '^',
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
        })
    }
}

impl Dir {
    fn vec(&self) -> Pos {
        match self {
            Dir::Up => [0, -1],
            Dir::Right => [1, 0],
            Dir::Down => [0, 1],
            Dir::Left => [-1, 0],
        }
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

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Input, Self::Err> {
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
        Ok(Input(Field { grid, pos }, dirs))
    }
}

impl Field {
    fn execute(mut self, dir: Dir) -> Self {
        let v = dir.vec();
        let p = vec2_add(self.pos, v);
        let mut cur = p;
        loop {
            if !self.grid.is_inside(cur) || self.grid[cur] == Cell::Wall {
                return self;
            }
            if self.grid[cur] == Cell::Clear {
                self.grid.swap(p, cur);
                self.pos = p;
                return self;
            };
            cur = vec2_add(cur, v);
        }
    }
}

fn main() {
    let input = include_str!("sample.txt");
    let Input(field, _) = input.parse().unwrap();
    println!("{field}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let f = Input::from_str(
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
        let Input(Field { grid, pos }, dirs) = f.unwrap();
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

    #[test]
    fn test_execute() {
        let Input(f, dirs) = r#"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
            "#
        .parse()
        .unwrap();
        let mut dir = dirs.into_iter();
        assert_eq!(
            f.to_string(),
            "########\n\
            #..O.O.#\n\
            ##@.O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #..O.O.#\n\
            ##@.O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #.@O.O.#\n\
            ##..O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #.@O.O.#\n\
            ##..O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #..@OO.#\n\
            ##..O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #...@OO#\n\
            ##..O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #...@OO#\n\
            ##..O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##..@..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##..@..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##.@...#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##.....#\n\
            #..@O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##.....#\n\
            #...@O.#\n\
            #.#.O..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##.....#\n\
            #....@O#\n\
            #.#.O..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##.....#\n\
            #.....O#\n\
            #.#.O@.#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##.....#\n\
            #.....O#\n\
            #.#O@..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
        let f = f.execute(dir.next().unwrap());
        assert_eq!(
            f.to_string(),
            "\
            ########\n\
            #....OO#\n\
            ##.....#\n\
            #.....O#\n\
            #.#O@..#\n\
            #...O..#\n\
            #...O..#\n\
            ########",
        );
    }
}
