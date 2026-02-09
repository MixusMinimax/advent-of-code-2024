use good_lp::{Solution, SolverModel, constraint, default_solver, variables};
use lazy_static::lazy_static;
use regex::Regex;
use vecmath::vec2_add;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct ClawMachine {
    button_a: [u32; 2],
    button_b: [u32; 2],
    prize: [u64; 2],
}

fn parse_claw_machines<'s>(
    lines: impl IntoIterator<Item = &'s str>,
) -> impl Iterator<Item = ClawMachine> {
    struct Iter<Lines> {
        lines: Lines,
    }

    lazy_static! {
        // language=regexp
        static ref BUTTON: Regex = Regex::new(r#"^Button [AB]: X\+(\d+), Y\+(\d+)$"#).unwrap();
        // language=regexp
        static ref PRIZE: Regex = Regex::new(r#"^Prize: X=(\d+), Y=(\d+)$"#).unwrap();
    }

    impl<'s, Lines: Iterator<Item = &'s str>> Iterator for Iter<Lines> {
        type Item = ClawMachine;

        fn next(&mut self) -> Option<Self::Item> {
            let [ax, ay] = BUTTON.captures(self.lines.next()?)?.extract().1;
            let [bx, by] = BUTTON.captures(self.lines.next()?)?.extract().1;
            let [px, py] = PRIZE.captures(self.lines.next()?)?.extract().1;
            Some(ClawMachine {
                button_a: [ax.parse().ok()?, ay.parse().ok()?],
                button_b: [bx.parse().ok()?, by.parse().ok()?],
                prize: [px.parse().ok()?, py.parse().ok()?],
            })
        }
    }

    Iter {
        lines: lines
            .into_iter()
            .map(str::trim)
            .filter(|s| !s.is_empty() && !s.starts_with("//")),
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Possible {
    a: u64,
    b: u64,
    tokens: u64,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Impossible;

fn minimize_button_pushes(
    &ClawMachine {
        button_a: [ax, ay],
        button_b: [bx, by],
        prize: [px, py],
    }: &ClawMachine,
) -> Result<Possible, Impossible> {
    variables! {
        vars:
            a (integer) >= 0;
            b (integer) >= 0;
    }
    let Ok(solution) = vars
        .minimise(a * 3 + b)
        .using(default_solver)
        .with(constraint!(a * ax + b * bx == px as f64))
        .with(constraint!(a * ay + b * by == py as f64))
        .solve()
    else {
        return Err(Impossible);
    };
    let av = solution.value(a).round() as _;
    let bv = solution.value(b).round() as _;
    if av * ax as u64 + bv * bx as u64 != px || av * ay as u64 + bv * by as u64 != py {
        return Err(Impossible);
    }
    Ok(Possible {
        a: av,
        b: bv,
        tokens: 3 * av + bv,
    })
}

fn main() {
    let input = include_str!("input.txt");

    let mut total = 0;
    let mut total_2 = 0;
    for mut claw_machine in parse_claw_machines(input.lines()) {
        println!("{claw_machine:?}");
        match minimize_button_pushes(&claw_machine) {
            Ok(Possible { a, b, tokens }) => {
                print!("a={} b={}; ", a, b);
                println!("3a + b = {}", tokens);
                total += tokens;
            }
            Err(Impossible) => {
                println!("Impossible");
            }
        }
        const V: [u64; 2] = [10000000000000, 10000000000000];
        claw_machine.prize = vec2_add(claw_machine.prize, V);
        match minimize_button_pushes(&claw_machine) {
            Ok(Possible { a, b, tokens }) => {
                print!("a={} b={}; ", a, b);
                println!("3a + b = {}", tokens);
                total_2 += tokens;
            }
            Err(Impossible) => {
                println!("Impossible");
            }
        }
        println!();
    }
    println!("Part1: {}", total);
    println!("Part2: {}", total_2);
}
