use crate::vec2::{Vec2, CARDINALS};
use crate::Day;

#[derive(Clone)]
pub struct Day12(Vec<Vec<Tile>>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Height(u8),
}

impl Tile {
    const MIN_HEIGHT: u8 = b'a';
    const MAX_HEIGHT: u8 = b'z';

    fn parse(c: char) -> Self {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            'a'..='z' => Self::Height(c as u8),
            _ => unreachable!(),
        }
    }

    const fn height(self) -> u8 {
        match self {
            Self::Start => Self::MIN_HEIGHT,
            Self::End => Self::MAX_HEIGHT,
            Self::Height(h) => h,
        }
    }
}

impl Day12 {
    fn find_end(&self) -> Vec2 {
        Vec2::map_coords(&self.0)
            .find(|pos| *pos.index(&self.0) == Tile::End)
            .unwrap()
    }

    fn shortest_path_back_to(&self, pred: impl Fn(Tile) -> bool) -> usize {
        let mut open = vec![self.find_end()];
        let mut visited = vec![0u128; self.0.len()];
        let mut dist = 0;
        loop {
            let mut new_open = Vec::new();
            for pos in open {
                if visited[pos.1 as usize] & (1 << pos.0) != 0 {
                    continue;
                }
                visited[pos.1 as usize] |= 1 << pos.0;
                if pred(*pos.index(&self.0)) {
                    return dist;
                }
                let height = pos.index(&self.0).height();
                for dir in CARDINALS {
                    let new_pos = pos + dir;
                    if height <= new_pos.index(&self.0).height() + 1 {
                        new_open.push(new_pos);
                    }
                }
            }
            open = new_open;
            dist += 1;
        }
    }
}

impl Day for Day12 {
    fn parse(input: &str) -> Self {
        let fence = Tile::Height(Tile::MIN_HEIGHT - 2);
        let mut map: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                std::iter::once(fence)
                    .chain(line.chars().map(Tile::parse))
                    .chain(Some(fence))
                    .collect()
            })
            .collect();
        map.push(vec![fence; map[0].len()]);
        map.insert(0, vec![fence; map[0].len()]);
        Self(map)
    }

    fn part1(&self) -> String {
        self.shortest_path_back_to(|tile| tile == Tile::Start)
            .to_string()
    }

    fn part2(&self) -> String {
        self.shortest_path_back_to(|tile| tile.height() == Tile::MIN_HEIGHT)
            .to_string()
    }
}
