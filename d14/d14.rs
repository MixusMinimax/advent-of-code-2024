use aoc2016::grid::{Pos, Size};
use itertools::Itertools;
use num::Integer;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::mem;
use vecmath::vec2_add;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

impl Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "p={},{} v={},{}",
            self.pos[0], self.pos[1], self.vel[0], self.vel[1]
        )
    }
}

mod parse {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, isize, line_ending, space1};
    use nom::combinator::map;
    use nom::multi::separated_list0;
    use nom::{IResult, Parser};

    #[inline]
    pub fn robot(s: &str) -> IResult<&str, Robot> {
        fn pos(s: &str) -> IResult<&str, Pos> {
            map((isize, char(','), isize), |(x, _, y)| [x, y]).parse(s)
        }
        map(
            (tag("p="), pos, space1, tag("v="), pos),
            |(_, pos, _, _, vel)| Robot { pos, vel },
        )
        .parse(s.trim())
    }

    #[inline]
    pub fn robots(s: &str) -> IResult<&str, Vec<Robot>> {
        separated_list0(line_ending, robot).parse(s.trim())
    }
}

#[inline]
fn move_robot(mut robot: Robot, size: Size) -> Robot {
    robot.pos = vec2_add(robot.pos, robot.vel);
    #[allow(clippy::needless_range_loop)]
    for i in 0..2 {
        robot.pos[i] = (robot.pos[i] % size[i] as isize + size[i] as isize) % size[i] as isize;
    }
    robot
}

fn safety_factor(robots: &[Robot], size: Size) -> usize {
    robots
        .iter()
        .map(|r| {
            match [
                r.pos[0].cmp(&((size[0] as isize) / 2)),
                r.pos[1].cmp(&((size[1] as isize) / 2)),
            ] {
                [Ordering::Less, Ordering::Less] => 0,
                [Ordering::Less, Ordering::Greater] => 1,
                [Ordering::Greater, Ordering::Less] => 2,
                [Ordering::Greater, Ordering::Greater] => 3,
                _ => 5,
            }
        })
        .counts()
        .into_iter()
        .filter(|(i, _)| (0..4).contains(i))
        .map(|(_, c)| c)
        .product()
}

fn display_robots(robots: &[Robot], size: Size) -> impl Display {
    struct S<'s>(&'s [Robot], Size);
    impl Display for S<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let Self(robots, [w, h]) = *self;
            let robots = robots.iter().map(|r| r.pos).counts();
            for y in 0..h as isize {
                if y > 0 {
                    writeln!(f)?;
                }
                for x in 0..w as isize {
                    write!(
                        f,
                        "{}",
                        match robots.get(&[x, y]) {
                            Some(&x @ 0..10) => (x as u8 + b'0') as char,
                            Some(_) => '?',
                            None => ' ',
                        },
                    )?;
                }
            }
            Ok(())
        }
    }
    S(robots, size)
}

fn part1(mut robots: Vec<Robot>, size: Size) {
    for _ in 0..100 {
        robots
            .iter_mut()
            .for_each(|r| *r = move_robot(mem::take(r), size));
    }
    let sf = safety_factor(&robots, size);
    println!("Part1: {}", sf);
}

fn part2(mut robots: Vec<Robot>, size: Size) {
    let mut f1 = File::create("d14/output.txt").unwrap();
    // let mut f = None;
    for s in 0..10000 {
        // if s.is_multiple_of(&100) {
        //     f = Some(File::create(format!("d14/output{s}-{}.txt", s + 99)).unwrap());
        // }
        // let f = f.as_mut().unwrap();
        // writeln!(f, "{s}s").unwrap();
        // writeln!(f, "{}", display_robots(&robots, size)).unwrap();
        if (s - 1).is_multiple_of(&103) {
            writeln!(f1, "{s}s").unwrap();
            writeln!(f1, "{}", display_robots(&robots, size)).unwrap();
        }
        robots
            .iter_mut()
            .for_each(|r| *r = move_robot(mem::take(r), size));
    }
}

fn main() {
    let input = include_str!("input.txt");
    let size = [101, 103];
    let robots = parse::robots(input).unwrap().1;
    part1(robots.clone(), size);
    part2(robots, size);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse::robots(
                r#"
                p=0,4 v=3,-3
                p=6,3 v=-1,-3
                p=10,3 v=-1,2
                "#,
            )
            .unwrap()
            .1,
            [
                Robot {
                    pos: [0, 4],
                    vel: [3, -3],
                },
                Robot {
                    pos: [6, 3],
                    vel: [-1, -3],
                },
                Robot {
                    pos: [10, 3],
                    vel: [-1, 2],
                },
            ],
        );
    }

    #[test]
    fn test_move_robot() {
        let size = [11, 7];
        let robot = parse::robot("p=2,4 v=2,-3").unwrap().1;
        assert_eq!(robot.to_string(), "p=2,4 v=2,-3");
        let robot = move_robot(robot, size);
        assert_eq!(robot.to_string(), "p=4,1 v=2,-3");
        let robot = move_robot(robot, size);
        assert_eq!(robot.to_string(), "p=6,5 v=2,-3");
        let robot = move_robot(robot, size);
        assert_eq!(robot.to_string(), "p=8,2 v=2,-3");
        let robot = move_robot(robot, size);
        assert_eq!(robot.to_string(), "p=10,6 v=2,-3");
        let robot = move_robot(robot, size);
        assert_eq!(robot.to_string(), "p=1,3 v=2,-3");
        let robot = move_robot(robot, size);
        assert_eq!(robot.to_string(), "p=3,0 v=2,-3");
        let robot = move_robot(robot, size);
        assert_eq!(robot.to_string(), "p=5,4 v=2,-3");
    }

    #[test]
    fn test_safety_factor() {
        let size = [11, 7];
        let mut robots = parse::robots(
            r#"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
            "#,
        )
        .unwrap()
        .1;
        for _ in 0..100 {
            robots
                .iter_mut()
                .for_each(|r| *r = move_robot(mem::take(r), size));
        }
        assert_eq!(safety_factor(&robots, size), 12);
    }
}
