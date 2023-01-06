use crate::Day;

#[derive(Clone)]
pub struct Day7 {
    folder_sizes: Vec<usize>,
    root_size: usize,
}

fn parse_folder<'a>(lines: &mut impl Iterator<Item = &'a str>, sizes: &mut Vec<usize>) -> usize {
    let mut size = 0;
    loop {
        let Some(line) = lines.next() else { break; };
        if line == "$ cd .." {
            break;
        }
        if line.starts_with("$ cd ") {
            size += parse_folder(lines, sizes);
        } else if !(line.starts_with('$') || line.starts_with('d')) {
            size += line.split_once(' ').unwrap().0.parse::<usize>().unwrap();
        }
    }
    sizes.push(size);
    size
}

impl Day for Day7 {
    fn parse(input: &str) -> Self {
        /*
        let mut stack: Vec<Vec<_>> = Vec::new();
        let mut folder_sizes = Vec::new();
        for line in input.lines() {
            if line == "$ cd .." {
                let size = stack.pop().unwrap().into_iter().sum();
                folder_sizes.push(size);
                stack.last_mut().unwrap().push(size);
            } else if line.starts_with("$ cd ") {
                stack.push(Vec::new());
            } else if !(line.starts_with('$') || line.starts_with('d')) {
                let size = line.split_once(' ').unwrap().0.parse::<usize>().unwrap();
                stack.last_mut().unwrap().push(size);
            }
        }
        while stack.len() > 1 {
            let size = stack.pop().unwrap().into_iter().sum();
            folder_sizes.push(size);
            stack.last_mut().unwrap().push(size);
        }
        let root_size = stack.pop().unwrap().into_iter().sum();
        folder_sizes.push(root_size);
        */
        let mut lines = input.lines();
        let mut folder_sizes = Vec::new();
        let root_size = parse_folder(&mut lines, &mut folder_sizes);
        Self {
            folder_sizes,
            root_size,
        }
    }

    fn part1(&self) -> String {
        self.folder_sizes
            .iter()
            .filter(|&n| n <= &100_000)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let free_up = self.root_size - 40_000_000;
        self.folder_sizes
            .iter()
            .filter(|&n| n >= &free_up)
            .min()
            .unwrap()
            .to_string()
    }
}
