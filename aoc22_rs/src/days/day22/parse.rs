#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Open,
    Closed,
    Void,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    pub fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid turn: {c}"),
        }
    }
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Closed,
            ' ' => Self::Void,
            _ => panic!("Invalid tile: {c}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Move(usize),
    Turn(Turn),
}

pub fn instructions(raw: &str) -> Vec<Instruction> {
    let mut distance = 0;
    let mut instructions = Vec::new();
    for c in raw.chars() {
        distance = match c {
            'L' | 'R' => {
                instructions.push(Instruction::Move(distance));
                instructions.push(Instruction::Turn(Turn::from_char(c)));
                0
            }
            '0'..='9' => distance * 10 + (c as usize - '0' as usize),
            _ => panic!("Invalid instruction: {c}"),
        }
    }
    instructions.push(Instruction::Move(distance));
    instructions
}
