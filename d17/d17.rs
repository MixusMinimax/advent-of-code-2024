use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use num::Integer;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::{mem, thread};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
struct Ignored(u8);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
struct Lit(u8);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(u8)]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    program: Vec<u8>,
    pc: isize,
}

fn parse_cpu(mut s: &str) -> Result<Cpu, nom::Err<nom::error::Error<&str>>> {
    use nom::IResult;
    use nom::Parser;
    use nom::bytes::complete::tag;
    use nom::character::complete::anychar;
    use nom::character::complete::{char, i64, space0, u8};
    use nom::combinator::{eof, map_res};
    use nom::multi::separated_list0;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use std::result::Result::Err;

    fn register(s: &str) -> IResult<&str, (char, i64)> {
        preceded(
            (tag("Register"), space0),
            (terminated(anychar, (char(':'), space0)), i64),
        )
        .parse(s.trim_start())
    }

    let mut cpu = Cpu::default();

    while let Ok((input, (c, v))) = register(s) {
        match c {
            'A' => cpu.a = v,
            'B' => cpu.b = v,
            'C' => cpu.c = v,
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    s,
                    nom::error::ErrorKind::Char,
                )));
            }
        }
        s = input;
    }

    cpu.program = map_res(
        (
            preceded(
                (tag("Program:"), space0),
                separated_list0((space0, char(','), space0), u8),
            ),
            eof,
        ),
        |(program, _)| {
            {
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
                    .collect::<Result<_, _>>()
            }
        },
    )
    .parse(s.trim())?
    .1;

    Ok(cpu)
}

impl Cpu {
    fn read(&self, op: ComboOp) -> i64 {
        match op {
            ComboOp::L0 => 0,
            ComboOp::L1 => 1,
            ComboOp::L2 => 2,
            ComboOp::L3 => 3,
            ComboOp::A => self.a,
            ComboOp::B => self.b,
            ComboOp::C => self.c,
            ComboOp::Reserved => panic!("Reserved!"),
        }
    }

    fn execute(mut self, mut output: impl FnMut(u8)) -> Self {
        assert!(self.pc.is_even());
        assert!(self.pc >= 0);
        assert!((self.pc as usize) < self.program.len());
        match Ins::from_bytes(
            self.program[self.pc as usize],
            self.program[self.pc as usize + 1],
        ) {
            Ins::Adv(op) => self.a >>= self.read(op),
            Ins::Bxl(Lit(v)) => self.b ^= v as i64,
            Ins::Bst(op) => self.b = self.read(op) & 0b111,
            Ins::Jnz(Lit(v)) => {
                if self.a != 0 {
                    self.pc = v as _;
                    return self;
                }
            }
            Ins::Bxc(_) => self.b ^= self.c,
            Ins::Out(op) => output(self.read(op).unsigned_abs() as u8 & 0b111),
            Ins::Bdv(op) => self.b = self.a >> self.read(op),
            Ins::Cdv(op) => self.c = self.a >> self.read(op),
        }
        self.pc += 2;
        self
    }

    fn run(self) -> (Self, Vec<u8>) {
        let mut output = vec![];
        let mut cpu = self;
        while (0..cpu.program.len() as isize).contains(&cpu.pc) {
            cpu = cpu.execute(|i| output.push(i));
        }
        (cpu, output)
    }
}

#[allow(dead_code)]
fn part_2_brute_force(cpu: Cpu) {
    let best = Arc::new(AtomicI64::new(i64::MAX));

    const THREADS: usize = 24;
    const CHECK_INTERVAL: i64 = 100_000;
    const PROGRESS_INTERVAL: u64 = 100_000;
    const START: i64 = 0x6020_0000_0000;
    const END: i64 = 0x8000_0000_0000;

    let m = MultiProgress::new();

    let mut handles = vec![];
    for offset in 0..THREADS {
        let best = best.clone();
        let cpu = cpu.clone();

        let pb = m.add(ProgressBar::new(END as u64 - START as u64));
        pb.set_style(
            ProgressStyle::with_template(&format!(
                "Thread {:02} [{{elapsed_precise}}] {{bar:40.cyan/blue}} {{pos:>12}}/{{len}} {{per_sec}} | {{msg}}",
                offset
            ))
                .unwrap()
        );

        handles.push(thread::spawn(move || {
            let mut cpu = cpu;
            let mut i = START + offset as i64;
            let mut local_progress: u64 = 0;
            let mut next_check = CHECK_INTERVAL;
            loop {
                if i >= next_check {
                    if i >= best.load(Ordering::Relaxed) {
                        break;
                    }
                    next_check = i + CHECK_INTERVAL;
                }

                cpu.a = i;
                let (_, output) = cpu.clone().run();
                if cpu.program == output {
                    best.fetch_min(i, Ordering::Relaxed);
                    pb.inc(local_progress);
                    pb.set_message(format!("i={:x}, l={}", i, output.len()));
                    break;
                }

                local_progress += 1;

                if local_progress >= PROGRESS_INTERVAL {
                    pb.inc(local_progress);
                    pb.set_message(format!("i={:x}, l={}", i, output.len()));
                    local_progress = 0;
                }

                i += THREADS as i64;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("A = {}", best.load(Ordering::Relaxed));
}

fn main() {
    let input = include_str!("input.txt");
    let cpu = parse_cpu(input).unwrap();
    let cpu2 = cpu.clone();
    println!(
        "{:?}",
        cpu.program
            .iter()
            .copied()
            .tuples()
            .map(|(a, b)| Ins::from_bytes(a, b))
            .collect::<Vec<_>>()
    );
    let (cpu, output) = cpu.run();
    println!("A = {}", cpu.a);
    println!("B = {}", cpu.b);
    println!("C = {}", cpu.c);
    println!("Output:");
    let mut cont = false;
    output.iter().for_each(|&i| {
        if cont {
            print!(",")
        } else {
            cont = true
        }
        print!("{}", i)
    });
    if cont {
        println!()
    }

    let mut cpu = cpu2;
    // cpu.a = 70368744177664;
    cpu.a = 0x602000000000;
    cpu.a = 0x602000000000;
    for _ in 0..100 {
        let (_, output) = cpu.clone().run();
        println!("A = {:x}", cpu.a);
        println!("{:?}", output);
        println!("{:?} | {:?}", cpu.program, output == cpu.program);
        cpu.a += 1;
    }

    // part_2_brute_force(cpu);
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
            cpu.program
                .into_iter()
                .tuples()
                .map(|(a, b)| Ins::from_bytes(a, b))
                .collect::<Vec<_>>(),
            [
                Ins::Adv(ComboOp::L1),
                Ins::Out(ComboOp::A),
                Ins::Jnz(Lit(0)),
            ],
        );
    }

    #[test]
    fn test_adv() {
        let cpu = parse_cpu(
            r"
            Register A: 9
            Program: 0,2
            ",
        )
        .unwrap();
        let cpu = cpu.execute(|_| {});
        assert_eq!(cpu.a, 9 / 4);
    }

    #[test]
    fn test_run_1() {
        let cpu = parse_cpu(
            r"
            Register C: 9
            Program: 2,6
            ",
        )
        .unwrap();
        let (cpu, output) = cpu.run();
        assert_eq!(
            cpu,
            Cpu {
                a: 0,
                b: 1,
                c: 9,
                program: vec![2, 6],
                pc: 2,
            }
        );
        assert_eq!(output, []);
    }

    #[test]
    fn test_run_2() {
        let cpu = parse_cpu(
            r"
            Register A: 10
            Program: 5,0,5,1,5,4
            ",
        )
        .unwrap();
        let (_, output) = cpu.run();
        assert_eq!(output, [0, 1, 2]);
    }

    #[test]
    fn test_run_3() {
        let cpu = parse_cpu(
            r"
            Register A: 2024
            Program: 0,1,5,4,3,0
            ",
        )
        .unwrap();
        let (cpu, output) = cpu.run();
        assert_eq!(cpu.a, 0);
        assert_eq!(output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn test_run_4() {
        let cpu = parse_cpu(
            r"
            Register B: 29
            Program: 1,7
            ",
        )
        .unwrap();
        let (cpu, _) = cpu.run();
        assert_eq!(cpu.b, 26);
    }

    #[test]
    fn test_run_5() {
        let cpu = parse_cpu(
            r"
            Register B: 2024
            Register C: 43690
            Program: 4,0
            ",
        )
        .unwrap();
        let (cpu, _) = cpu.run();
        assert_eq!(cpu.b, 44354);
    }

    #[test]
    fn test_run_sample() {
        let cpu = parse_cpu(
            r"
            Register A: 729
            Program: 0,1,5,4,3,0
            ",
        )
        .unwrap();
        let (_, output) = cpu.run();
        assert_eq!(output, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
