use crate::Day;

#[derive(Clone)]
struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

#[derive(Clone)]
pub struct Day5 {
    crates: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

#[inline]
fn top_crates(crates: Vec<Vec<char>>) -> String {
    crates
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect()
}

impl Day for Day5 {
    fn parse(input: &str) -> Self {
        let (raw_crates, raw_instrs) = input.split_once("\n\n").unwrap();
        let mut crate_lines = raw_crates.lines().rev();
        let num_crates = (crate_lines.next().unwrap().len() + 1) / 4;
        let mut crates = vec![Vec::new(); num_crates];
        for line in crate_lines {
            for (stack_index, crate_) in line.chars().skip(1).step_by(4).enumerate() {
                if crate_ != ' ' {
                    crates[stack_index].push(crate_);
                }
            }
        }
        let instructions = raw_instrs
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace().skip(1).step_by(2); // skip the words between the numbers
                let count = parts.next().unwrap().parse().unwrap();
                let source = parts.next().unwrap().parse::<usize>().unwrap() - 1; // convert 1-indexed to 0-indexed
                let dest = parts.next().unwrap().parse::<usize>().unwrap() - 1;
                Instruction {
                    count,
                    source,
                    dest,
                }
            })
            .collect();
        Self {
            crates,
            instructions,
        }
    }

    fn part1(&self) -> String {
        let mut crates = self.crates.clone();
        for instr in &self.instructions {
            let source = &mut crates[instr.source];
            let mut stack = source.split_off(source.len() - instr.count);
            stack.reverse();
            crates[instr.dest].append(&mut stack);
        }
        top_crates(crates)
    }

    fn part2(&self) -> String {
        let mut crates = self.crates.clone();
        for instr in &self.instructions {
            let source = &mut crates[instr.source];
            let mut stack = source.split_off(source.len() - instr.count);
            crates[instr.dest].append(&mut stack);
        }
        top_crates(crates)
    }
}
