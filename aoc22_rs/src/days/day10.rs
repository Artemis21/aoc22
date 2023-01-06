use crate::Day;

#[derive(Clone)]
pub struct Day10(Vec<isize>);

impl Day for Day10 {
    fn parse(input: &str) -> Self {
        let mut x = 1;
        let mut values = Vec::new();
        for line in input.lines() {
            values.push(x);
            if line != "noop" {
                values.push(x);
                x += line.split_once(' ').unwrap().1.parse::<isize>().unwrap();
            }
        }
        Self(values)
    }

    fn part1(&self) -> String {
        (0..6)
            .map(|i| 20 + i * 40)
            .map(|clock| clock as isize * self.0[clock - 1])
            .sum::<isize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut screen = [[false; 40]; 6];
        for (clock, x_reg) in self.0.iter().enumerate() {
            let (row, col) = (clock / 40, clock % 40);
            let diff = col as isize - x_reg;
            if (-1..=1).contains(&diff) {
                screen[row][col] = true;
            }
        }
        let ascii_screen = screen
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&x| if x { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        advent_of_code_ocr::parse_string_to_letters(&ascii_screen)
    }
}
