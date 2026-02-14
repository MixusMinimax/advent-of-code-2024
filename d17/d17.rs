use nom::IResult;
use std::mem;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
struct Ignored(u8);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
struct Lit(u8);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(u8)]
enum Ins {
    Adv(ComboOp) = 0,
    Bxl(Lit) = 1,
    Bst(ComboOp) = 2,
    Jnz(Lit) = 3,
    Bxc(Ignored) = 4,
    Out(ComboOp) = 5,
    Bdv(ComboOp) = 6,
    Cdv(ComboOp) = 7,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(u8)]
enum ComboOp {
    L0 = 0,
    L1 = 1,
    L2 = 2,
    L3 = 3,
    A = 4,
    B = 5,
    C = 6,
    Reserved = 7,
}

impl Ins {
    fn from_bytes(op_code: u8, op: u8) -> Self {
        assert!(op_code < 8);
        assert!(op < 8);
        unsafe { mem::transmute([op_code, op]) }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Cpu {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<Ins>,
    pc: usize,
}

fn parse_cpu(s: &str) -> Result<Cpu, nom::Err<nom::error::Error<&str>>> {
    use itertools::Itertools;
    use nom::Parser;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, i64, line_ending, space0, space1, u8};
    use nom::combinator::{eof, map_res};
    use nom::multi::{many0, separated_list0};
    use nom::sequence::{delimited, preceded};
    use std::result::Result::Err;
    fn nl(s: &str) -> IResult<&str, &str> {
        line_ending(s)
    }
    map_res(
        (
            delimited((space0, tag("Register A:"), space1), i64, (space0, nl)),
            delimited((space0, tag("Register B:"), space1), i64, (space0, nl)),
            delimited((space0, tag("Register C:"), space1), i64, (space0, nl)),
            many0((space0, nl)),
            preceded(
                (space0, tag("Program:")),
                separated_list0(char(','), delimited(space0, u8, space0)),
            ),
            eof,
        ),
        |(a, b, c, _, program, _)| {
            Ok::<_, nom::Err<nom::error::Error<&str>>>(Cpu {
                a,
                b,
                c,
                program: {
                    program
                        .into_iter()
                        .map(|i| {
                            if i < 8 {
                                Ok(i)
                            } else {
                                Err(nom::Err::<nom::error::Error<&str>>::Error(
                                    nom::error::Error::new(s, nom::error::ErrorKind::Digit),
                                ))
                            }
                        })
                        .tuples()
                        .map(|(a, b)| a.and_then(|a| b.map(|b| Ins::from_bytes(a, b))))
                        .collect::<Result<_, _>>()?
                },
                pc: 0,
            })
        },
    )
    .parse(s.trim())
    .map(|(_, cpu)| cpu)
}

fn main() {
    let input = include_str!("input.txt");
    let cpu = parse_cpu(input).unwrap();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn test_ins_from_bytes() {
        assert_eq!(Ins::from_bytes(5, 4), Ins::Out(ComboOp::A));
        assert_eq!(Ins::from_bytes(0, 0), Ins::Adv(ComboOp::L0));
        assert!(catch_unwind(|| Ins::from_bytes(8, 0)).is_err());
        assert!(catch_unwind(|| Ins::from_bytes(0, 8)).is_err());
    }

    #[test]
    fn test_parse() {
        let cpu = parse_cpu(
            r"
            Register A: 729
            Register B: 0
            Register C: 0

            Program: 0,1,5,4,3,0
            ",
        );
        assert!(cpu.is_ok());
        let cpu = cpu.unwrap();
        assert_eq!(cpu.a, 729);
        assert_eq!(cpu.b, 0);
        assert_eq!(cpu.c, 0);
        assert_eq!(
            cpu.program,
            [
                Ins::Adv(ComboOp::L1),
                Ins::Out(ComboOp::A),
                Ins::Jnz(Lit(0)),
            ],
        );
    }
}
