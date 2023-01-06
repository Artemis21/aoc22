use crate::{util::max_n, Day};

#[derive(Debug, Clone)]
pub struct Day11 {
    monkeys: Vec<Monkey>,
    worry_modulo: usize,
}

impl Day11 {
    fn run_rounds<const DIV: usize>(&mut self, rounds: usize) -> usize {
        let mut activity = vec![0; self.monkeys.len()];
        for _ in 0..rounds {
            for (idx, activity) in activity.iter_mut().enumerate() {
                let monkey = &mut self.monkeys[idx];
                let desc = monkey.description;
                for item in monkey.items.drain(..).collect::<Vec<_>>() {
                    *activity += 1;
                    let item = (desc.operation.eval(item) / DIV) % self.worry_modulo;
                    let to = if item % desc.test_divisor == 0 {
                        desc.if_true
                    } else {
                        desc.if_false
                    };
                    self.monkeys[to].items.push(item);
                }
            }
        }
        max_n::<2, _>(activity).into_iter().product()
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    description: MonkeyDescription,
}

#[derive(Debug, Clone, Copy)]
struct MonkeyDescription {
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
    fn parse(raw: &'static str) -> Self {
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
        let description = MonkeyDescription {
            operation,
            test_divisor,
            if_true,
            if_false,
        };
        Self { items, description }
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
        let monkeys: Vec<_> = raw.split("\n\n").map(Monkey::parse).collect();
        let worry_modulo = monkeys.iter().map(|m| m.description.test_divisor).product();
        Self {
            monkeys,
            worry_modulo,
        }
    }

    fn part1(&self) -> String {
        self.clone().run_rounds::<3>(20).to_string()
    }

    fn part2(&self) -> String {
        self.clone().run_rounds::<1>(10_000).to_string()
    }
}
