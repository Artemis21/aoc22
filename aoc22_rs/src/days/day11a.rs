use crate::{util::max_n, Day};

#[derive(Debug, Clone)]
pub struct Day11 {
    items: Vec<Vec<usize>>,
    monkeys: Vec<Monkey>,
    worry_modulo: usize,
}

impl Day11 {
    fn run_rounds<const DIV: usize>(&self, rounds: usize) -> usize {
        let mut activity = vec![0; self.monkeys.len()];
        for (start_monkey, items) in self.items.iter().enumerate() {
            for item in items {
                let mut worry = *item;
                let mut monkey = start_monkey;
                let mut rounds_left = rounds;
                while rounds_left > 0 {
                    activity[monkey] += 1;
                    let monkey_spec = &self.monkeys[monkey];
                    worry = (monkey_spec.operation.eval(worry) / DIV) % self.worry_modulo;
                    let new_monkey = if worry % monkey_spec.test_divisor == 0 {
                        monkey_spec.if_true
                    } else {
                        monkey_spec.if_false
                    };
                    if new_monkey <= monkey {
                        rounds_left -= 1;
                    }
                    monkey = new_monkey;
                }
            }
        }
        max_n::<2, _>(activity).into_iter().product()
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    operation: Operation,
    test_divisor: usize,
    if_true: usize,
    if_false: usize,
}

const STARTING_ITEMS_OFFSET: usize = "  Starting items: ".len();
const OPERATION_OFFSET: usize = "  Operation: new = ".len();
const TEST_DIVISOR_OFFSET: usize = "  Test: divisible by ".len();
const IF_TRUE_OFFSET: usize = "    If true: throw to monkey ".len();
const IF_FALSE_OFFSET: usize = "    If false: throw to monkey ".len();

impl Monkey {
    fn parse(raw: &'static str) -> (Self, Vec<usize>) {
        let mut lines = raw.lines();
        lines.next();
        let items = lines.next().unwrap()[STARTING_ITEMS_OFFSET..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let operation = Operation::parse(&lines.next().unwrap()[OPERATION_OFFSET..]);
        let test_divisor = lines.next().unwrap()[TEST_DIVISOR_OFFSET..]
            .parse()
            .unwrap();
        let if_true = lines.next().unwrap()[IF_TRUE_OFFSET..]
            .parse::<usize>()
            .unwrap();
        let if_false = lines.next().unwrap()[IF_FALSE_OFFSET..]
            .parse::<usize>()
            .unwrap();
        (Self {
            operation,
            test_divisor,
            if_true,
            if_false,
        }, items)
    }
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    left: Operand,
    right: Operand,
    op: Operator,
}

impl Operation {
    fn parse(raw: &'static str) -> Self {
        let mut parts = raw.split(' ');
        let left = Operand::parse(parts.next().unwrap());
        let op = Operator::parse(parts.next().unwrap());
        let right = Operand::parse(parts.next().unwrap());
        Self { left, right, op }
    }

    const fn eval(&self, input: usize) -> usize {
        self.op.eval(self.left.eval(input), self.right.eval(input))
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Input,
    Literal(usize),
}

impl Operand {
    fn parse(raw: &'static str) -> Self {
        match raw {
            "old" => Self::Input,
            _ => Self::Literal(raw.parse().unwrap()),
        }
    }

    const fn eval(&self, input: usize) -> usize {
        match self {
            Self::Input => input,
            Self::Literal(lit) => *lit,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(raw: &'static str) -> Self {
        match raw {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => panic!("Unknown operator: {raw}"),
        }
    }

    const fn eval(self, left: usize, right: usize) -> usize {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
        }
    }
}

impl Day for Day11 {
    fn parse(raw: &'static str) -> Self {
        let (monkeys, items): (Vec<_>, Vec<_>) = raw.split("\n\n").map(Monkey::parse).unzip();
        let worry_modulo = monkeys.iter().map(|m| m.test_divisor).product();
        Self {
            monkeys,
            items,
            worry_modulo,
        }
    }

    fn part1(&self) -> String {
        self.run_rounds::<3>(20).to_string()
    }

    fn part2(&self) -> String {
        self.run_rounds::<1>(10_000).to_string()
    }
}
