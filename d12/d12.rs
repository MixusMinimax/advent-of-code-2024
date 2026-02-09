use aoc2016::grid::Grid;
use vecmath::{vec2_add, vec2_sub};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Area {
    plant: u8,
    area: u32,
    perimeter: u32,
}

impl Area {
    fn cost(&self) -> u64 {
        self.area as u64 * self.perimeter as u64
    }
}

fn get_areas(grid: &Grid<u8>, combine_straights: bool) -> Vec<Area> {
    let mut visited = Grid::<bool>::new(grid.size);
    let mut areas = Vec::new();

    for p in grid.positions() {
        if visited[p] {
            continue;
        }
        let plant = grid[p];

        visited[p] = true;
        let mut frontier = vec![p];
        let mut area = 0;
        let mut perimeter = 0;
        while let Some(cur @ [x, y]) = frontier.pop() {
            area += 1;
            for n in [[x + 1, y], [x, y + 1], [x - 1, y], [x, y - 1]] {
                if grid.is_inside(n) && grid[n] == plant {
                    if !visited[n] {
                        visited[n] = true;
                        frontier.push(n);
                    }
                } else if combine_straights {
                    // only the right-most edges count, if viewed from inside
                    let [vx, vy] = vec2_sub(n, cur);
                    let vr = [-vy, vx];
                    let r = vec2_add(cur, vr);
                    let rn = vec2_add(n, vr);
                    if !grid.is_inside(r)
                        || grid[r] != plant
                        || grid.is_inside(rn) && grid[rn] == plant
                    {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        areas.push(Area {
            plant,
            area,
            perimeter,
        });
    }
    areas
}

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::from_lines(input.lines(), |_, c| c as u8);
    let areas = get_areas(&grid, false);
    let cost: u64 = areas.iter().map(Area::cost).sum();
    println!("Part1: {}", cost);

    let areas = get_areas(&grid, true);
    let cost: u64 = areas.iter().map(Area::cost).sum();
    println!("Part2: {}", cost);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_areas() {
        let grid = Grid::from([*b"AAAA", *b"BBCD", *b"BBCC", *b"EEEC"]);
        let areas = get_areas(&grid, false);
        assert_eq!(
            areas,
            vec![
                Area {
                    plant: b'A',
                    area: 4,
                    perimeter: 10,
                },
                Area {
                    plant: b'B',
                    area: 4,
                    perimeter: 8,
                },
                Area {
                    plant: b'C',
                    area: 4,
                    perimeter: 10,
                },
                Area {
                    plant: b'D',
                    area: 1,
                    perimeter: 4,
                },
                Area {
                    plant: b'E',
                    area: 3,
                    perimeter: 8,
                },
            ],
        );
    }

    #[test]
    fn test_areas_bulk() {
        let grid = Grid::from([*b"AAAA", *b"BBCD", *b"BBCC", *b"EEEC"]);
        let areas = get_areas(&grid, true);
        assert_eq!(
            areas,
            vec![
                Area {
                    plant: b'A',
                    area: 4,
                    perimeter: 4,
                },
                Area {
                    plant: b'B',
                    area: 4,
                    perimeter: 4,
                },
                Area {
                    plant: b'C',
                    area: 4,
                    perimeter: 8,
                },
                Area {
                    plant: b'D',
                    area: 1,
                    perimeter: 4,
                },
                Area {
                    plant: b'E',
                    area: 3,
                    perimeter: 4,
                },
            ],
        );
    }
}
