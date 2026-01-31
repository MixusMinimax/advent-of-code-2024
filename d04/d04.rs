use itertools::Itertools;
use vecmath::{vec2_add, vec2_scale};

struct Grid<'i> {
    data: Vec<&'i [u8]>,
    width: usize,
    height: usize,
}

fn part1(
    &Grid {
        ref data,
        width,
        height,
    }: &Grid,
) {
    const DIRECTIONS: [[isize; 2]; 8] = [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ];
    let mut total_count = 0;
    for y in 0..height {
        for x in 0..width {
            if data[y][x] != b'X' {
                continue;
            }
            total_count += DIRECTIONS
                .iter()
                .copied()
                .filter(|&[dx, dy]| {
                    (match dx {
                        0 => true,
                        -1 => x >= 3,
                        1 => x < width - 3,
                        _ => unreachable!(),
                    }) && (match dy {
                        0 => true,
                        -1 => y >= 3,
                        1 => y < height - 3,
                        _ => unreachable!(),
                    })
                })
                .filter(|&dir| {
                    (1..4)
                        .map(|i| vec2_add([x as isize, y as isize], vec2_scale(dir, i)))
                        .map(|[x, y]| &data[y as usize][x as usize])
                        .eq(b"MAS")
                })
                .count();
        }
    }
    println!("Part1: {total_count}");
}

fn part2(
    &Grid {
        ref data,
        width,
        height,
    }: &Grid,
) {
    let mut total_count = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if let (b"MAS" | b"SAM", b"MAS" | b"SAM") = (
                &[data[y - 1][x - 1], data[y][x], data[y + 1][x + 1]],
                &[data[y - 1][x + 1], data[y][x], data[y + 1][x - 1]],
            ) {
                total_count += 1
            }
        }
    }
    println!("Part2: {total_count}");
}

fn main() {
    // let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let grid = {
        let data: Vec<_> = input.lines().map(str::as_bytes).collect();
        assert!(!data.is_empty());
        assert!(data.iter().map(|s| s.len()).all_equal());
        let width = data[0].len();
        let height = data.len();
        Grid {
            data,
            width,
            height,
        }
    };
    part1(&grid);
    part2(&grid);
}
