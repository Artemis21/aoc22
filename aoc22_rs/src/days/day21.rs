use rustc_hash::FxHashMap;

use crate::Day;

#[derive(Clone)]
pub struct Day21(Operation);

#[derive(Clone)]
enum Monkey {
    Leaf(isize),
    Human(isize),
    Node(Operation),
}

impl Monkey {
    fn new(raw_monkeys: &FxHashMap<&'static str, RawMonkey>, name: &'static str) -> Self {
        match raw_monkeys[name] {
            RawMonkey::Leaf(value) => Self::Leaf(value),
            RawMonkey::Human(value) => Self::Human(value),
            RawMonkey::Node(RawOperation { lhs, rhs, op }) => {
                let lhs = Box::new(Self::new(raw_monkeys, lhs));
                let rhs = Box::new(Self::new(raw_monkeys, rhs));
                Self::Node(Operation { lhs, rhs, op })
            }
        }
    }

    fn eval(&self) -> isize {
        match self {
            Self::Leaf(value) | Self::Human(value) => *value,
            Self::Node(op) => op.eval(),
        }
    }

    fn eval_no_human(&self) -> Option<isize> {
        match self {
            Self::Leaf(value) => Some(*value),
            Self::Human(_) => None,
            Self::Node(op) => op.eval_no_human(),
        }
    }

    fn solve_for_human(&self, result: isize) -> isize {
        match self {
            Self::Leaf(_) => panic!("cannot solve tree for human because no human exists"),
            Self::Human(_) => result,
            Self::Node(op) => op.solve_for_human(result),
        }
    }
}

#[derive(Clone)]
struct Operation {
    lhs: Box<Monkey>,
    rhs: Box<Monkey>,
    op: Operator,
}

impl Operation {
    fn eval(&self) -> isize {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();
        self.op.apply(lhs, rhs)
    }

    fn eval_no_human(&self) -> Option<isize> {
        let lhs = self.lhs.eval_no_human()?;
        let rhs = self.rhs.eval_no_human()?;
        Some(self.op.apply(lhs, rhs))
    }

    fn solve_for_human(&self, result: isize) -> isize {
        let lhs = self.lhs.eval_no_human();
        let rhs = self.rhs.eval_no_human();
        match (lhs, rhs) {
            (Some(lhs), None) => self.rhs.solve_for_human(self.op.solve_for_rhs(lhs, result)),
            (None, Some(rhs)) => self.lhs.solve_for_human(self.op.solve_for_lhs(rhs, result)),
            _ => panic!("exactly one operand must expand to a human"),
        }
    }

    fn solve_for_human_as_eq(&self) -> isize {
        let lhs = self.lhs.eval_no_human();
        let rhs = self.rhs.eval_no_human();
        match (lhs, rhs) {
            (Some(lhs), None) => self.rhs.solve_for_human(lhs),
            (None, Some(rhs)) => self.lhs.solve_for_human(rhs),
            _ => panic!("exactly one operand must expand to a human"),
        }
    }
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn parse(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mul,
            '/' => Self::Div,
            _ => panic!("Invalid operator: {c}"),
        }
    }

    const fn apply(self, lhs: isize, rhs: isize) -> isize {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }

    const fn solve_for_lhs(self, rhs: isize, result: isize) -> isize {
        match self {
            Self::Add => result - rhs,
            Self::Sub => result + rhs,
            Self::Mul => result / rhs,
            Self::Div => result * rhs,
        }
    }

    const fn solve_for_rhs(self, lhs: isize, result: isize) -> isize {
        match self {
            Self::Add => result - lhs,
            Self::Sub => lhs - result,
            Self::Mul => result / lhs,
            Self::Div => lhs / result,
        }
    }
}

struct RawOperation {
    lhs: &'static str,
    rhs: &'static str,
    op: Operator,
}

impl RawOperation {
    fn parse(input: &'static str) -> Self {
        let mut parts = input.split(' ');
        let lhs = parts.next().unwrap();
        let op = Operator::parse(parts.next().unwrap().chars().next().unwrap());
        let rhs = parts.next().unwrap();
        Self { lhs, rhs, op }
    }
}

enum RawMonkey {
    Leaf(isize),
    Human(isize),
    Node(RawOperation),
}

impl RawMonkey {
    fn parse(input: &'static str) -> (&'static str, Self) {
        let (name, def) = input.split_once(": ").unwrap();
        let monkey = if name == "humn" {
            Self::Human(def.parse().unwrap())
        } else if def.contains(' ') {
            Self::Node(RawOperation::parse(def))
        } else {
            Self::Leaf(def.parse().unwrap())
        };
        (name, monkey)
    }
}

impl Day for Day21 {
    fn parse(input: &'static str) -> Self {
        let raw_monkeys: FxHashMap<_, _> = input.lines().map(RawMonkey::parse).collect();
        let root = Monkey::new(&raw_monkeys, "root");
        let Monkey::Node(op) = root else { panic!("root is not an operation") };
        Self(op)
    }

    fn part1(&self) -> String {
        self.0.eval().to_string()
    }

    fn part2(&self) -> String {
        self.0.solve_for_human_as_eq().to_string()
    }
}
