//! We use panics in these functions instead of [Results](Result), because it is not a public library
//! and is only used in single-run binary targets. Panics result in better backtraces and
//! are easier to write with, since [Results](Result) or [Options](Option) have no benefits here.

pub mod grid {
    use std::fmt::{Display, Formatter};
    use std::iter::once;
    use std::ops::{Index, IndexMut};

    pub type Pos = [isize; 2];
    pub type Size = [usize; 2];

    pub fn idx([x, y]: [isize; 2], [width, height]: [usize; 2]) -> usize {
        assert!((0..width as isize).contains(&x));
        assert!((0..height as isize).contains(&y));
        y as usize * width + x as usize
    }

    #[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
    pub struct Grid<Cell> {
        pub cells: Vec<Cell>,
        pub size: Size,
    }

    impl<Cell> Grid<Cell> {
        pub fn from_lines<'s>(
            lines: impl IntoIterator<Item = &'s str>,
            mut create_cell: impl FnMut(Pos, char) -> Cell,
        ) -> Self {
            let mut it = lines.into_iter().map(str::trim).filter(|s| !s.is_empty());
            let first = it.next().unwrap();
            let width = first.len();
            let data: Vec<_> = once(first)
                .chain(it.inspect(|l| debug_assert_eq!(l.len(), width)))
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(move |(x, c)| ([x as isize, y as isize], c))
                })
                .map(|(p, c)| create_cell(p, c))
                .collect();
            let height = data.len() / width;
            debug_assert_eq!(width * height, data.len());
            Self {
                cells: data,
                size: [width, height],
            }
        }
    }

    impl<Cell> Grid<Cell> {
        pub fn is_inside(&self, [x, y]: Pos) -> bool {
            (0..self.size[0] as isize).contains(&x) && (0..self.size[1] as isize).contains(&y)
        }

        pub fn width(&self) -> usize {
            self.size[0]
        }

        pub fn height(&self) -> usize {
            self.size[1]
        }
    }

    impl<Cell> Index<[isize; 2]> for Grid<Cell> {
        type Output = Cell;

        fn index(&self, index: [isize; 2]) -> &Self::Output {
            let i = idx(index, self.size);
            &self.cells[i]
        }
    }

    impl<Cell> IndexMut<[isize; 2]> for Grid<Cell> {
        fn index_mut(&mut self, index: [isize; 2]) -> &mut Self::Output {
            let i = idx(index, self.size);
            &mut self.cells[i]
        }
    }

    impl<Cell> Grid<Cell> {
        pub fn map<NewCell>(self, map_fn: impl Fn(Pos, Cell) -> NewCell) -> Grid<NewCell> {
            let width = self.width();
            Grid {
                cells: self
                    .cells
                    .into_iter()
                    .enumerate()
                    .map(move |(i, c)| map_fn([(i % width) as isize, (i / width) as isize], c))
                    .collect(),
                size: self.size,
            }
        }
    }

    mod display {
        use super::*;
        use std::fmt;
        use std::fmt::{Display, Formatter};

        struct GridDisplay<'g, Cell, DisplayFn>(&'g Grid<Cell>, DisplayFn);

        impl<Cell, DisplayFn: Fn(&Cell, &mut Formatter) -> fmt::Result> Display
            for GridDisplay<'_, Cell, DisplayFn>
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                for (i, c) in self.0.cells.iter().enumerate() {
                    if i != 0 && i.is_multiple_of(self.0.width()) {
                        writeln!(f)?;
                    }
                    self.1(c, f)?;
                }
                Ok(())
            }
        }

        impl<Cell> Grid<Cell> {
            pub fn display(
                &self,
                format: impl Fn(&Cell, &mut Formatter) -> fmt::Result,
            ) -> impl Display {
                GridDisplay(self, format)
            }
        }

        struct GridDisplayPos<'g, Cell, DisplayFn>(&'g Grid<Cell>, DisplayFn);

        impl<Cell, DisplayFn: Fn(Pos, &Cell, &mut Formatter) -> fmt::Result> Display
            for GridDisplayPos<'_, Cell, DisplayFn>
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                for (i, c) in self.0.cells.iter().enumerate() {
                    if i != 0 && i.is_multiple_of(self.0.width()) {
                        writeln!(f)?;
                    }
                    self.1(
                        [(i % self.0.width()) as isize, (i / self.0.width()) as isize],
                        c,
                        f,
                    )?;
                }
                Ok(())
            }
        }

        impl<Cell> Grid<Cell> {
            pub fn display_pos(
                &self,
                format: impl Fn(Pos, &Cell, &mut Formatter) -> fmt::Result,
            ) -> impl Display {
                GridDisplayPos(self, format)
            }
        }
    }

    impl<Cell: Display> Display for Grid<Cell> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.display(<Cell as Display>::fmt).fmt(f)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use core::fmt::Display;

        #[test]
        fn test() {
            let g = Grid::from_lines(["1234", "5678", "9012"], |_, c| c);
            println!("{}", g.display(Display::fmt));
            println!("{}", g);
        }
    }
}
